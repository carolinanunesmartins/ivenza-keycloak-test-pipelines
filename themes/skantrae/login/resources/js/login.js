// Add js here
window.addEventListener(
    'DOMContentLoaded',
    function () {
        var els = document.getElementsByClassName('kc-feedback-text');
        for (var el of els) {
            var text = el.innerText || el.textContent;
            if (text === 'U ontvangt binnenkort een e-mail met verdere instructies.') {
                document.write('U wordt omgeleid');
                window.location.href = 'https://expert.skantrae.com/account/login?successPasswordReset=true';
            }
        }
    },
    false,
);
