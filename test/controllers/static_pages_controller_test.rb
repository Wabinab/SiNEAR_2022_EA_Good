require "test_helper"

class StaticPagesControllerTest < ActionDispatch::IntegrationTest
  test "should get root" do 
    get root_url
    assert_response :success
  end

  test "should get home" do
    get home_url
    assert_response :success
  end

  test "should get donate" do 
    get donate_url 
    assert_response :success
  end
end
