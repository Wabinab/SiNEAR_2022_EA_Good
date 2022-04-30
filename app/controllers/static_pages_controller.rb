class StaticPagesController < ApplicationController
  def home
  end

  def donate 
    @donate_target = [
      ["Against Malaria Foundation: ", "d-malaria"],
      ["Deworming Programs", "d-worming"],
      ["GiveDirectly", "d-directly"]
    ]
  end
end
