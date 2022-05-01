class StaticPagesController < ApplicationController
  def home
  end

  def donate 
    @suffix_token_id = Time.now.to_f.to_s.gsub('.', '_') + '_' + ('a'..'z').to_a.shuffle[0, 5].join
  end
end
