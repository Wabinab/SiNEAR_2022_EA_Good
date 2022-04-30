Rails.application.routes.draw do

  get 'users', to: 'users#index'
  get '/users/:account_id', to: 'users#show'
  post 'users', to: 'users#create'
  # get '@:account_id', to: 'users#index'

  root 'users#new'

  resources :users
end
