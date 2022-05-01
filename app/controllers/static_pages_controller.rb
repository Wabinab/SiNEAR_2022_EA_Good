class StaticPagesController < ApplicationController
  def home
  end

  def donate 
    @donate_target = [
      ["Against Malaria Foundation: ", "d_malaria"],
      ["Deworming Programs", "d_worming"],
      ["GiveDirectly", "d_directly"]
    ]
  end
end
