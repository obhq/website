document.addEventListener('DOMContentLoaded', function () {
    init();
    adjustScreenSize();

    window.addEventListener('resize', adjustScreenSize);
});