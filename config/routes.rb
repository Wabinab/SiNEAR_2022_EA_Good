Rails.application.routes.draw do
  get 'home', to: 'static_pages#home'
  get 'donate', to: 'static_pages#donate'

  get 'users', to: 'users#new'
  get 'admin', to: 'users#index'
  get '/users/:account_id', to: 'users#show'
  post 'users', to: 'users#create'
  # get '@:account_id', to: 'users#index'

  root 'static_pages#home'

  resources :users
end
