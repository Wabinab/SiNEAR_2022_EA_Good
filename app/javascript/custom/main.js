import { connect, Contract, keyStores, WalletConnection, utils } from 'near-api-js';
import getConfig from './config.js';


const nearConfig = getConfig('development', 'ea_nft.wabinab.testnet')
const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));

window.nearConfig = nearConfig
window.near = near

window.walletConnection = new WalletConnection(near)

window.accountId = window.walletConnection.getAccountId()

window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
  viewMethods: ['get_id_by_category'],
  changeMethods: ['generate_template', 'minting_interface'],
})


function logout() {
  window.walletConnection.signOut()
  window.location.replace(window.location.origin + window.location.pathname)
}

function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName)
}


function generate_template() {
    var title = document.getElementById("template_title").value;
    var description = document.getElementById("template_desc").value;
    var media = document.getElementById("card_img").value;

    var template_id = document.getElementById("template_id").value;

    window.contract.generate_template(
      {
        "template_id": template_id,
        "metadata": {
          "title": title,
          "description": description,
          "media": media,
        }
      },
      "30000000000000",
      utils.format.parseNearAmount("0.1")
    ).then(
      window.location.reload()
    );
}


function minting_interface(suffix_token_id) {
    window.contract.get_id_by_category().then(
      (a_hashMap) => {
        var hash_of_amounts = {};
        var sum = 0.1;  // for storage.

        for (var key in a_hashMap) {
          let id = parseInt(a_hashMap[key]);
          let amount = parseFloat(document.getElementById(key).value);

          if (!isNaN(amount)) {
            hash_of_amounts[id] = amount;
            sum += (amount + 0.1);
          }
        }

        window.contract.minting_interface(
          {
            "suffix_token_id": suffix_token_id,
            "hash_of_amounts": hash_of_amounts,
            "issued_at": Math.floor(Date.now() / 1000)
          },
          "300000000000000",  // 300 TGas
          utils.format.parseNearAmount(sum.toPrecision(2)),
        ).then(
          window.location.reload()
        );
      }
    )
    
}





window.generate_template = generate_template
window.minting_interface = minting_interface
window.logout = logout
window.login = login