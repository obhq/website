// This script has required functions that *most* pages use, e.g. the insertion of the header or the dynamic size adjuster

let menuState = true;
let Timer = 0;
// let url_prefix = "/obhqWebsite/public_html/Gamma-Boi"
let avifSupport;

// Adjust screen size for mobile and 4k monitors for some reason
function adjustScreenSize() {
    let screenWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
    let referenceWidth = 1920;
    let referenceFontSize = 16;
    let phoneReferenceWidth = 700;

    if (screenWidth >= referenceWidth) { // 2000 ++
        let fontSize = (screenWidth / referenceWidth) * referenceFontSize;
        document.documentElement.style.fontSize = fontSize + 'px';

    } else if (screenWidth >= phoneReferenceWidth) { // 720 - 2000
        document.documentElement.style.fontSize = referenceFontSize + 'px';

    } else { // -- 720
        let fontSize = (screenWidth / phoneReferenceWidth) * referenceFontSize;
        document.documentElement.style.fontSize = fontSize + 'px';
    }

    if (document.getElementById("headerIcon")) {
        let headerIcon = document.getElementById("headerIcon");

        if (screenWidth < "370") {
            // no point to check for avif here, since the favicon already loads the png version :/
            headerIcon.src = "/_images/Obliteration-smoll.png";

        } else {
            // check avif support
            if (avifSupport && headerIcon.src !== ("/_images/Obliteration.avif")) {
                headerIcon.src = "/_images/Obliteration.avif";

            } else if (!avifSupport && headerIcon.src !== ("/_images/Obliteration.png")) {
                headerIcon.src = "/_images/Obliteration.png";
            }
        }
    }
}


// Adds a shadow to the header when scrolling
function headerShadow() {
    requestAnimationFrame(() => {
        const header = document.querySelector('header');

        if (window.scrollY > 0) {
            header.style.boxShadow = "0 1px 5px rgb(0, 0, 0, 0.2)";
        } else {
            header.style.boxShadow = '';
        }
    });
}

// used for the hamburger menu button [on mobile]
function menuButton() {
    const menu = document.getElementById("menu");
    const menuButton = document.getElementById("menuButton");

    if (menuState === true) { // if its open, close it
        menuState = false;
        menuButton.src = "/_images/close.svg";
        menu.classList.remove("NoOpacity")

    } else if (menuState === false) { // if its closed, open it
        menuState = true;
        menuButton.src = "/_images/menu.svg";
        menu.classList.add("NoOpacity");
    }
}

// Used to animate elements that become visible
function animationHandler() {
    const observer = new IntersectionObserver((entries, observer) => {
        entries.forEach(entry => {
            console.log(entry);
            if (entry.isIntersecting) {
                entry.target.classList.add("animate");
            } else {
                entry.target.classList.remove("animate");
            }
        });
    }, {threshold: 0.6});

    document.querySelectorAll('.toAnimate').forEach(element => {
        observer.observe(element);
    });
}

function avifSupportCheck() {
    return new Promise(resolve => {
        const avif = new Image();
        avif.src = "data:image/avif;base64,AAAAIGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZk1BMUIAAADybWV0YQAAAAAAAAAoaGRscgAAAAAAAAAAcGljdAAAAAAAAAAAAAAAAGxpYmF2aWYAAAAADnBpdG0AAAAAAAEAAAAeaWxvYwAAAABEAAABAAEAAAABAAABGgAAAB0AAAAoaWluZgAAAAAAAQAAABppbmZlAgAAAAABAABhdjAxQ29sb3IAAAAAamlwcnAAAABLaXBjbwAAABRpc3BlAAAAAAAAAAIAAAACAAAAEHBpeGkAAAAAAwgICAAAAAxhdjFDgQ0MAAAAABNjb2xybmNseAACAAIAAYAAAAAXaXBtYQAAAAAAAAABAAEEAQKDBAAAACVtZGF0EgAKCBgANogQEAwgMg8f8D///8WfhwB8+ErK42A=";

        avif.onload = function () {
            console.log('AVIF IS SUPPORTED :D');
            resolve(true);
        };

        avif.onerror = function () {
            console.warn('AVIF IS NOT SUPPORTED D:');
            resolve(false);
        };
    });
}

async function init() {
    avifSupport = await avifSupportCheck();

    let header_html = `
        <header class="header">
            <a class="headerIcon NoOpacity" href="/">
                <img id="headerIcon" src="/_images/Obliteration.avif" height="40px" width="auto" alt="Obliteration logo">
            </a>
            <div class="headerRight NoOpacity">
                <a class="headerLink" href="./download">Download</a>
                <a class="headerLink" href="./compatibility">Compatibility</a>
                <a class="headerLink" href="./wiki">Wiki</a>
                <div class="headerRightIcons">
                    <a href="https://github.com/obhq/obliteration" target="_blank" style="display: flex;">
                        <img class="headerRightIconGithub" src="/_images/github.svg" alt="GitHub logo">
                    </a>
                    <a href="https://discord.gg/Qsdaxj6tnH" target="_blank" style="display: flex;">
                        <img class="headerRightIconDiscord" src="/_images/discord.svg" alt="Discord logo">
                    </a>
                    <img class="headerRightIconMenu" src="/_images/menu.svg" onclick="menuButton()" id="menuButton" alt="Mobile menu button">
                </div>
            </div>
        </header>`;

    let menu_html = `
        <div class="menuContainer NoOpacity" id="menu" onclick="menuButton()">
            <div class="menu">
                <a class="menuLink" href="./download">Download</a>
                <a class="menuLink" href="./compatibility">Compatibility</a>
                <a class="menuLink" href="https://github.com/obhq/obliteration/wiki">Wiki</a>
            </div>
        </div>`;

    document.getElementById("header").outerHTML = header_html;
    document.getElementById("menu").outerHTML = menu_html;

    // Animation, 
    let imagesLoaded = 0;
    const images = document.querySelector(".header").querySelectorAll("img");

    images.forEach(image => {
        image.addEventListener("load", () => {
            imagesLoaded++;
            if (imagesLoaded === images.length) {
                requestAnimationFrame(() => {

                    document.querySelector(".headerIcon").classList.remove("NoOpacity");
                    document.querySelector(".headerRight").classList.remove("NoOpacity");
                });
            }
        })
    })

    return avifSupport;
}