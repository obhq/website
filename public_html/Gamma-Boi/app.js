document.addEventListener('DOMContentLoaded', function () {
    init();
    adjustScreenSize();
    headerShadow();

    window.addEventListener('resize', adjustScreenSize);
    window.addEventListener("scroll", headerUpdate);


    // fetch('/stats.json').then(response => response.json()).then(jsonData => {
    //     document.getElementById("stars").textContent = jsonData.stars;
    //     document.getElementById("issues").textContent = jsonData.issues;
    //     document.getElementById("devbuilds").textContent = jsonData.devbuilds;
    // })
});

function buttonScroll() {
    const targetElement = document.getElementById('main2');
    targetElement.scrollIntoView({behavior: 'smooth'});
}

