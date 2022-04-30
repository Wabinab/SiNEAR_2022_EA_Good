Rails.application.routes.draw do
  get 'home', to: 'static_pages#home'

  get 'users', to: 'users#index'
  get '/users/:account_id', to: 'users#show'
  post 'users', to: 'users#create'
  # get '@:account_id', to: 'users#index'

  root 'static_pages#home'

  resources :users
end
