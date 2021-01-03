import('../pkg/index.js')
    .catch(console.error)
    .then(function ({ boot }) {
        boot(document.querySelector('#app'))
    });