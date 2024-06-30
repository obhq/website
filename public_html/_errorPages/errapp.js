document.addEventListener('DOMContentLoaded', async function () {
    // required.js
    adjustScreenSize(700)
    
    await init();
    adjustScreenSize(700);

    if (document.getElementById("goBackButton")) {
        document.getElementById("goBackButton").addEventListener("click", () => history.back())
    } else if (document.getElementById("reloadButton")) {
        document.getElementById("reloadButton").addEventListener("click", () => location.reload())
    }

    window.addEventListener('resize', () => {
        adjustScreenSize(700);
    });
});