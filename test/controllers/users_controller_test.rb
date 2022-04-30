require "test_helper"

class UsersControllerTest < ActionDispatch::IntegrationTest
  # test "should get new" do
  #   get users_url
  #   assert_response :success
  # end

  test "should get index" do 
    get admin_url
    assert_response :success
  end
end
