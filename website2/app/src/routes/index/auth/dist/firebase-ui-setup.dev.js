"use strict";

firebase.auth().onAuthStateChanged(function (user) {
  if (user) {
    user.getIdToken().then(function (token) {
      var ev = new CustomEvent("user", {
        detail: {
          displayName: user.displayName,
          email: user.email,
          photoURL: user.photoURL,
          providerId: user.providerId,
          uid: user.uid,
          token: token
        }
      });
      window.dispatchEvent(ev);
    });
  } else {
    window.dispatchEvent(new CustomEvent("user", {
      detail: null
    }));
  }
});
window.addEventListener("logout", function _callee(ev) {
  return regeneratorRuntime.async(function _callee$(_context) {
    while (1) {
      switch (_context.prev = _context.next) {
        case 0:
          _context.next = 2;
          return regeneratorRuntime.awrap(firebase.auth().signOut());

        case 2:
          window.location.reload();

        case 3:
        case "end":
          return _context.stop();
      }
    }
  });
});
var ui = new firebaseui.auth.AuthUI(firebase.auth());
ui.start('#firebase-auth', {
  signInOptions: [{
    provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
    signInMethod: firebase.auth.EmailAuthProvider.EMAIL_LINK_SIGN_IN_METHOD
  }, firebase.auth.GoogleAuthProvider.PROVIDER_ID, firebase.auth.TwitterAuthProvider.PROVIDER_ID, "apple.com"],
  signInFlow: "popup",
  callbacks: {
    signInSuccessWithAuthResult: function signInSuccessWithAuthResult(authResult, redirectUrl) {
      var ev = new CustomEvent("signin", authResult),
          modal = document.querySelector("l-modal[id='login']");

      if (modal) {
        modal.open = false;
      }

      window.dispatchEvent(ev);
    }
  }
});