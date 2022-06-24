firebase.auth().onAuthStateChanged(function (user) {
  if (user) {
    user.getIdToken().then(token => {
      const ev = new CustomEvent("user", {
        detail: {
          displayName: user.displayName,
          email: user.email,
          photoURL: user.photoURL,
          providerId: user.providerId,
          uid: user.uid,
          token
        }
      });

      window.dispatchEvent(ev);
    });
  } else {
    window.dispatchEvent(new CustomEvent("user", { detail: null }));
  }
});

window.addEventListener("logout", (ev) => {
  console.log("caught logout event", ev.detail);
  firebase.auth().signOut();
});

const ui = new firebaseui.auth.AuthUI(firebase.auth());
ui.start('#firebase-auth', {
  signInOptions: [
    {
      provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
      signInMethod: firebase.auth.EmailAuthProvider.EMAIL_LINK_SIGN_IN_METHOD
    },
    firebase.auth.GoogleAuthProvider.PROVIDER_ID,
    firebase.auth.TwitterAuthProvider.PROVIDER_ID,
    "apple.com",
  ],
  signInFlow: "popup",
  callbacks: {
    signInSuccessWithAuthResult(authResult, redirectUrl) {
      const ev = new CustomEvent("signin", authResult),
        modal = document.querySelector("l-modal[id='login']");
      if (modal) {
        modal.open = false;
      }
      window.dispatchEvent(ev);
    }
  }
});
