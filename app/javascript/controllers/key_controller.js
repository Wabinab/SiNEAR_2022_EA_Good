import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = [ "accountid", "allkey", "publickey" ]

  static values = {allkeys: String, publickey: String, accountid: String}

  initialize() {
    this.show()
  }

  show() {
    this.allkeysValue = window.walletConnection._authData.allKeys[0];
    this.accountidValue = window.walletConnection._authData.accountId;
    this.publickeyValue = "nil";
    this.showAllKeys();
  }

  generate() {
    window.location.replace(
      window.location.href 
      + '?account_id=' + this.accountidValue
      + '&public_key=' + this.publickeyValue
      + '&all_keys=' + this.allkeysValue
    );
  }

  admin() {
    window.location.replace(
      window.location.origin
      + "/admin"
      + '?account_id=' + this.accountidValue
      + '&all_keys=' + this.allkeysValue
    );
  }

  showAllKeys() {
    this.allkeyTargets.forEach((element, _index) => {
      element.innerText = "All Keys: " + this.allkeysValue;
    })

    this.accountidTargets.forEach((element, _index) => {
      element.innerText = "Account ID: " + this.accountidValue;
    })

    this.publickeyTargets.forEach((element, _index) => {
      element.innerText = "Public Key: " + this.publickeyValue;
    })
  }
}