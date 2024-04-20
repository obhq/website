document.addEventListener('DOMContentLoaded', function () {
    init();
    adjustScreenSize();
    headerShadow();
    animationHandler();

    window.addEventListener('resize', adjustScreenSize);
    window.addEventListener("scroll", headerUpdate);

    fetch('/stats.json').then(response => response.json()).then(jsonData => {
        countingAnimation(jsonData.stars, document.getElementById("starsNumber"));
        countingAnimation(jsonData.issues, document.getElementById("issuesNumber"));
        countingAnimation(jsonData.devbuilds, document.getElementById("devbuildsNumber"));
    })
});

function buttonScroll() {
    const targetElement = document.getElementById('main2');
    targetElement.scrollIntoView({behavior: 'smooth'});
}


function countingAnimation(number, element) {
    const digits = number.toString().split("");

    digits.forEach(digit => {
        let spanList = '';

        for (let i = 0; i < 10; i++) {
            spanList += `<span class="midText text-themed">${i}</span>`;
        }
        
        element.innerHTML += `<span class="tableNumbersContainer" style="transform: translateY(-1000%)">${spanList}`;
    });

    setTimeout(() => {
        element.querySelectorAll('.tableNumbersContainer').forEach((e, i) => {
            e.style.transform = `translateY(-${100 * parseInt(digits[i])}%)`;
        });
    }, 100)
}