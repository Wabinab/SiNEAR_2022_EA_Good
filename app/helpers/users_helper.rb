require 'near_api'

module UsersHelper

  def define_constants
    @node_url = 'https://rpc.testnet.near.org/'
    @conf = NearApi::Config.new(node_url: @node_url)
    @query = NearApi::Query.new(config = @conf)

    @contract = 'ea_nft.wabinab.testnet'
  end

  def gravatar_for(user)
    gravatar_id = Digest::MD5::hexdigest(user.account_id)
    gravatar_url = "https://secure.gravatar.com/avatar/#{gravatar_id}?d=identicon&r=PG"
    image_tag(gravatar_url, class: "gravatar")
  end

  def get_templates
    data = @query.function(
      @contract,
      'view_metadatas',
      {}
    )["result"]["result"]
    
    if data.nil?
      {}
    else
      JSON.parse(data.pack('c*'))  
    end
    
  end

  def get_list_to_donate 
    data = @query.function(
      @contract,
      'get_list_to_donate',
      {}
    )["result"]["result"]
    
    if data.nil?
      {}
    else 
      JSON.parse(data.pack('c*'))
    end
  end
end
