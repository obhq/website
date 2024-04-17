document.addEventListener('DOMContentLoaded', function () {
    init();
    adjustScreenSize();
    headerShadow();
    animationHandler();

    window.addEventListener('resize', adjustScreenSize);
    window.addEventListener("scroll", headerUpdate);

    fetch('/stats.json').then(response => response.json()).then(jsonData => {
        countingAnimation(jsonData.stars, document.getElementById("stars"));
        countingAnimation(jsonData.issues, document.getElementById("issues"));
        countingAnimation(jsonData.devbuilds, document.getElementById("devbuilds"));
    })
});

function buttonScroll() {
    const targetElement = document.getElementById('main2');
    targetElement.scrollIntoView({behavior: 'smooth'});
}


function countingAnimation(number, element) {
    const digits = number.toString().split("");
    var first = true;

    digits.forEach(digit => {
        let spanList = '';

        for (let i = 0; i < 10; i++) {
            spanList += `<span class=" midText text-themed">${i}</span>`;
        }

        if (first) {
            first = false;
            element.innerHTML = `<span style="transform: translateY(-1000%)" id="silly">${spanList}`;
        } else {
            element.innerHTML += `<span style="transform: translateY(-1000%)" id="silly">${spanList}`;
        }
    });

    setTimeout(() => {
        element.querySelectorAll('#silly').forEach((e, i) => {
            e.style.transform = `translateY(-${100 * parseInt(digits[i])}%)`;
        });
    }, 100)
}