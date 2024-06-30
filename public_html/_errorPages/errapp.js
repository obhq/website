document.addEventListener('DOMContentLoaded', async function () {
    await init();
    adjustScreenSize();

    if (document.getElementById("goBackButton")) {
        document.getElementById("goBackButton").addEventListener("click", () => history.back())
    } else if (document.getElementById("reloadButton")) {
        document.getElementById("reloadButton").addEventListener("click", () => location.reload())
    }

    window.addEventListener('resize', adjustScreenSize);
});