import { connect, Contract, keyStores, WalletConnection, utils } from 'near-api-js';
import getConfig from './config.js';


const nearConfig = getConfig('development', 'ea_nft.wabinab.testnet')
const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));

window.nearConfig = nearConfig
window.near = near

window.walletConnection = new WalletConnection(near)

window.accountId = window.walletConnection.getAccountId()

window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
  changeMethods: ['generate_template', 'set_greeting_for_others'],
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


function set_greeting_for_others(target) {
  var message = document.getElementById("someone_message").value;
  window.contract.set_greeting_for_others({
    "target": target,
    "message": message
  }).then(
    value => {
      alert("Successful called set_greeting_for_others.");
      window.location.reload();
    },
    err => alert(err),
  );
}



window.generate_template = generate_template
window.set_greeting = set_greeting
window.set_greeting_for_others = set_greeting_for_others
window.logout = logout
window.login = login