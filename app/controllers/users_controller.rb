# Requires more test cases. Haven't understand how to test properly, though. 

class UsersController < ApplicationController

  def new
    @user = User.new
  end

  def show 
    @user = User.new 
    @user.account_id = search_params[:account_id].gsub('-', '.')
  end

  def index
    all_keys = index_params[:all_keys]
    if authenticate(all_keys)
      redirect_back fallback_location: root_path
    else
      render 'index'
    end
  end
  

  def create 
    @user = User.new(user_params)

    if @user.save 
      # redirect is done later. 
    else
      @user =  User.find_by(account_id: user_params[:account_id])
      @user.public_key = user_params[:public_key]
      @user.all_keys = user_params[:all_keys]
      @user.save
    end

    redirect_to @user
    
  end

  private 

    def user_params 
      params.require(:user).permit(:account_id, :public_key, :all_keys)
    end

    def index_params 
      params.permit(:account_id, :all_keys)
    end

    def search_params 
      params.permit(:account_id)
    end

    def authenticate(all_keys)
      authorized_id = "somebodyelse.testnet"
      @user = User.find_by(account_id: authorized_id)
  
      if @user.all_keys == all_keys
        true
      else
        false
      end
    end
end
