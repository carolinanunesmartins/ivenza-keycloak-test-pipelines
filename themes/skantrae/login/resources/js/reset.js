window.addEventListener(
    'DOMContentLoaded',
    function () {
        var queryString = window.location.search;
        var urlParams = new URLSearchParams(queryString);
        var email = urlParams.get('email');

        if (email) {
            document.getElementById('kc-reset-password-form').style.visibility = 'hidden';
            var elInput = document.getElementById('username');
            elInput.value = email;
            var elButton = document.getElementById('resetPasswordSubmit');
            elButton.click();
        }
    },
    false,
);
