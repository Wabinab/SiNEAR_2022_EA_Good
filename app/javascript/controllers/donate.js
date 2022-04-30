import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = [ "slide", "name", "donate" ]

  static values = {current: Boolean}

  initialize() {
    this.currentValue = window.walletConnection.isSignedIn()
    this.showCurrentLogin()
    this.showCurrentName()
    this.showCurrentDonate()
  }

  switch() {
    if (this.currentValue) {
      window.logout()
    } else {
      window.login()
    }

    this.currentValue = !this.currentValue
    this.showCurrentLogin()
  }

  showCurrentLogin() {
    this.slideTargets.forEach((element, _index) => {
      if (this.currentValue) {
        element.innerText = "Logout"
      } else {
        element.innerText = "Login"
      }
    })
  }
}