document.addEventListener('DOMContentLoaded', async function () {
    // required.js
    adjustScreenSize(); // might not change the top image for on mobile

    await init();
    adjustScreenSize(700); // ensure change of top image
    headerShadow();
    animationHandler(700);

    window.addEventListener('resize', () => {
        adjustScreenSize(700);
    });
    window.addEventListener("scroll", headerShadow);


    await fetch('stats.json').then(response => response.json()).then(jsonData => {
        countingAnimation(jsonData.stars, document.getElementById("starsNumber"));
        countingAnimation(jsonData.issues, document.getElementById("issuesNumber"));
        countingAnimation(jsonData.devbuilds, document.getElementById("devbuildsNumber"));
    });

    setTimeout(() => {
        document.querySelectorAll(".tableNumbersContainer").forEach(element => {
            element.style.transition = "none";
        });
    }, 1900); // timed with the transition speed in css
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