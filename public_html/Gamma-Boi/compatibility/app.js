let fancyJsonData; // :3
let totalPages;
let tagFilter = [];
let oldestFilter = false;

// Check Avif Support
console.log(`Hey there! I've implemented an avif support check to spare browsers like Edge from having a stroke :D!`);
const avif = new Image();
avif.src = "data:image/avif;base64,AAAAIGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZk1BMUIAAADybWV0YQAAAAAAAAAoaGRscgAAAAAAAAAAcGljdAAAAAAAAAAAAAAAAGxpYmF2aWYAAAAADnBpdG0AAAAAAAEAAAAeaWxvYwAAAABEAAABAAEAAAABAAABGgAAAB0AAAAoaWluZgAAAAAAAQAAABppbmZlAgAAAAABAABhdjAxQ29sb3IAAAAAamlwcnAAAABLaXBjbwAAABRpc3BlAAAAAAAAAAIAAAACAAAAEHBpeGkAAAAAAwgICAAAAAxhdjFDgQ0MAAAAABNjb2xybmNseAACAAIAAYAAAAAXaXBtYQAAAAAAAAABAAEEAQKDBAAAACVtZGF0EgAKCBgANogQEAwgMg8f8D///8WfhwB8+ErK42A=";
avif.onload = function () {
    console.log('AVIF IS SUPPORTED :D');
    avifSupport = true;
};
avif.onerror = function () {
    console.log('AVIF IS NOT SUPPORTED D:');
    avifSupport = false;
    window.alert("Hey! Your browser doesn't support avif, avif is an image format that has low file sizes while having high quality images. You won't get any game images");
};


document.addEventListener('DOMContentLoaded', function () {
    init();
    adjustScreenSize();
    headerShadow();

    window.addEventListener('resize', adjustScreenSize);
    window.addEventListener("scroll", headerUpdate);

    // + fetch issues and set the tag bars
    fetch('../_scripts/search.php?q=&stats')  //todo : change
        .then(response => response.json())
        .then(jsonData => {
            gameCardHandler(jsonData);
            fancyJsonData = jsonData;
            // pageButtonHandler();

            totalPages = jsonData.info.pages;
            totalTime = jsonData.info.time;
            totalIssues = jsonData.info.issues;
            console.log("\nCOMPATIBILITY STATS");

            // stats & tag filter
            jsonData.stats.forEach(stat => {
                var tag = stat.tag;
                var percent = stat.percent;
                var tagBar = document.getElementById(tag + 'Bar');
                var tagContainer = tagBar.parentElement;
                console.log(`${tag} = ${percent}% [${stat.count}]`);

                tagBar.style.width = percent + '%';
                document.getElementById(tag + 'Percent').textContent = percent + '%';
                document.getElementById(tag + 'Number').textContent = stat.count;

                tagContainer.addEventListener('click', function () {
                    tagContainer.classList.toggle('compStatusSelected');
                    tagFilter.includes(tag) ? tagFilter.splice(tagFilter.indexOf(tag), 1) : tagFilter.push(tag);
                    pageNumber = 1;
                    updateSearchResults();
                });
            });
            console.log("\n");
        })
        .catch(console.error);
});

// Searching
document.getElementById('search').addEventListener('input', function () {
    pageNumber = 1;
    updateSearchResults();
});

async function updateSearchResults() {
    clearTimeout(Timer);
    const cardsContainer = document.getElementById("CardsContainer");
    const searchQuery = document.getElementById('search').value;

    cardsContainer.querySelectorAll(".gameCardS, .gameCard, .gameCardE").forEach(container => {
        const skeletonDiv = document.createElement('a');

        container.classList.forEach(name => {
            skeletonDiv.classList.add(name)
        });

        skeletonDiv.classList.add("skeletonLoading");
        cardsContainer.replaceChild(skeletonDiv, container);
    });

    Timer = setTimeout(() => {
        fetch('../_scripts/search.php?q=' + searchQuery + '&tag=' + tagFilter + '&page=' + pageNumber + '&oldest=' + oldestFilter)
            .then(response => response.json())
            .then(jsonData => {
                fancyJsonData = jsonData;
                gameCardHandler(jsonData);
                // pageButtonHandler("");
            })
            .catch(console.error);
    }, 300);
}


// Game Card handler
function gameCardHandler(jsonData) {
    const gameWrapper = document.getElementById("CardsContainer");
    gameWrapper.innerHTML = "";

    let currentNum = 0;
    let lastNum = Object.keys(jsonData.games).length;

    jsonData.games.forEach(game => {
        currentNum++;
        // game image URL
        let imageSource;
        let cardType;
        let imageText = "N/A";
        let imageTextSize = 1.38;

        switch (true) {
            case game.image && avifSupport && game.type === "HB":
                imageSource = "https://obliteration.net/_images/HB/" + game.title + ".avif";
                break;
            case game.image && avifSupport:
                imageSource = "https://obliteration.net/_images/games/" + game.code + ".avif";
                break;
        }

        if (game.type === "HB") { // needs to be applied to all homebrews
            imageText = "HOME<br>BREW";
            imageTextSize = 1.25;
        } else if (game.type === "SYS") {
            imageText = "SYSTEM";
            imageTextSize = 1.13;
        }

        switch (true) {
            case currentNum === 1 :
                cardType = "gameCardS";
                break;
            case currentNum === lastNum :
                cardType = "gameCardE";
                break;
        }

        // game cards
        const gameElementHTML = `
    <a class="${cardType ? cardType : `gameCard`} ${game.tag}" target="_blank" href="https://github.com/obhq/compatibility/issues/${game.id}">
        ${imageSource ? `<img class="gameCardImage" loading="lazy" alt="${game.title} - ${game.code} game image" src="${imageSource}">` : `<p class='gameCardImageText' style='font-size: ${imageTextSize}rem;'>${imageText}</p>`}
        <div class="gameContent">
            <p class="gameCardTitle">${game.title}</p>
            <p class="gameCardCode">${game.code}</p>
            <p class="gameCardTag">${game.tag}</p>
            <p class="gameCardUpdated">${game.upDate}</p>
        </div>
    </a>`;
        let cardClass = "." + (cardType ? cardType : `gameCard`);

        const tempContainer = document.createElement('a');
        tempContainer.innerHTML = gameElementHTML;
        let gameContainer = tempContainer.querySelector(cardClass);
        gameWrapper.appendChild(gameContainer);
    });

    document.getElementById("infoText").innerText = `${jsonData.info.issues} results in ${jsonData.info.time}ms`;
}


// game cards and stats handler and other onload stuff
