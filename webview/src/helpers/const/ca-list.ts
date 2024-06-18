export const CaList = {
    letsencrypt: {
        short: 'letsencrypt',
        name: 'Let’s Encrypt',
        url: 'https://acme-v02.api.letsencrypt.org/directory',
    },
    letsencrypt_test: {
        short: 'letsencrypt_test',
        name: 'Let’s Encrypt (Test)',
        url: 'https://acme-staging-v02.api.letsencrypt.org/directory',
    },
    buypass: {
        short: 'buypass',
        name: 'BuyPass.com CA',
        url: 'https://api.buypass.com/acme/directory',
    },
    buypass_test: {
        short: 'buypass_test',
        name: 'BuyPass.com CA (Test)',
        url: 'https://api.test4.buypass.no/acme/directory',
    },
    zerossl: {
        short: 'zerossl',
        name: 'ZeroSSL.com CA',
        url: 'https://acme.zerossl.com/v2/DV90',
    },
    sslcom: {
        short: 'sslcom',
        name: 'SSL.com CA',
        url: 'https://acme.ssl.com/sslcom-dv-ecc',
    },
    sslcom_rsa: {
        short: 'sslcom',
        name: 'SSL.com CA (RSA)',
        url: 'https://acme.ssl.com/sslcom-dv-rsa',
    },
    google: {
        short: 'google',
        name: 'Google Public CA',
        url: 'https://dv.acme-v02.api.pki.goog/directory',
    },
    google_test: {
        short: 'googletest',
        name: 'Google Public CA (Test)',
        url: 'https://dv.acme-v02.test-api.pki.goog/directory',
    },
};
