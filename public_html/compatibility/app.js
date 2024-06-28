let databaseJsonData; // :3
let totalPages; // is this needed? like genuinely?
let currentPage = 1; // starts from 1, NOT 0
let tagFilter = [];
let justUpdated;
let issuesPerPage = 10;
let issuesCount;
// let Timer;
const codeRegex = /[a-zA-Z]{4}[0-9]{5}/;

document.addEventListener('DOMContentLoaded', async function () {
    let startTime = performance.now()

    // required.js
    adjustScreenSize(610); // might not change the top image for on mobile
    await init();
    adjustScreenSize(610); // ensure change of top image
    headerShadow();

    window.addEventListener('resize', () => {
        adjustScreenSize(610);
    });
    window.addEventListener("scroll", headerShadow);

    // if no avifSupport
    if (!avifSupport && !document.cookie.split("; ").find(c => c.startsWith(name))) {
        alert("Your browser doesn't seem to support AVIF.\nAVIF allows obliteration to display a lot of images without high file sizes. Thus you won't see any game images. \nThis is the only time you will receive this message.");

        let date = new Date();
        date.setMonth(date.getMonth() + 1);
        document.cookie = "avifMessage; Expires=" + date.toUTCString();
    }

    // + fetch issues and set the tag bars
    fetch('database.json')
        .then(response => response.json())
        .then(jsonData => {
            setTimeout(() => {
                let totalIssues = jsonData.length;

                databaseJsonData = jsonData;
                issuesCount = totalIssues;
                totalPages = Math.ceil(totalIssues / issuesPerPage);

                gameCardHandler(jsonData.slice(0, issuesPerPage)); // first 10
                PageSelectorUpdater();

                // stats && tag filter
                console.log("\nCOMPATIBILITY STATS");

                let availableTags = ['Nothing', 'Boots', 'Intro', 'InGame', 'Playable'];
                let totalPercentage = 0;
                let naCount = 0;

                let tagPercentages = [];
                let tagCount = [];

                // get tag count && raw tag percentages
                availableTags.forEach(tag => {
                    tagCount[tag] = 0; // init tag count

                    jsonData.forEach(issue => {
                        if (issue.tag === tag) {
                            tagCount[tag]++;

                        } else if (issue.tag === "N/A") {
                            naCount++;
                        }
                    });

                    let rawPercentage = parseFloat(((tagCount[tag] / (totalIssues - naCount)) * 100).toFixed(2));
                    tagPercentages[tag] = rawPercentage;
                    totalPercentage += rawPercentage;
                });

                // make percentages correct (mostly ~heh)
                if (totalPercentage !== 100) {

                    let difference = (100 - totalPercentage);
                    tagPercentages["Nothing"] = parseFloat(tagPercentages["Nothing"] + difference).toFixed(2);
                }


                // go through each tag and set's the tag bars
                availableTags.forEach(tag => {
                    let StatusWrapper = document.getElementById(tag + "Status")

                    // iterates through the children of the StatusWrapper, then it iterates through their class lists
                    Array.from(StatusWrapper.children).forEach(child => child.classList.forEach(name => {
                        switch (name) {
                            case "compMenuStatusLabel":
                                child.classList.remove("NoOpacity");
                                break;
                            case "compMenuStatusInfo":
                                child.textContent = tagPercentages[tag] + '%';
                                child.classList.remove("NoOpacity");
                                break;
                            case "compMenuStatusAmount":
                                child.textContent = tagCount[tag];
                                break;
                            case "compMenuStatusBar":
                                child.style.width = tagPercentages[tag] + '%';
                                child.classList.remove("NoOpacity");
                        }
                    }));

                    console.log(`${tag} = ${tagPercentages[tag]}% [${tagCount[tag]}]`);

                    // updates the tagFilter when a [tag bar] is clicked
                    StatusWrapper.addEventListener('click', function () {
                        StatusWrapper.classList.toggle('compStatusSelected');
                        tagFilter.includes(tag) ? tagFilter.splice(tagFilter.indexOf(tag), 1) : tagFilter.push(tag);

                        currentPage = 1;
                        updateSearchResults();
                    });
                });
                console.log("\n");

            }, 300 - (performance.now() - startTime)); // fake delay to make animations not look like shit lmao
        })
        .catch(console.error);
});


// Searching in the game menu
function OnCompMenuSearch() {
    currentPage = 1;
    updateSearchResults();
}


// it searches the json and updates any needed variables
function updateSearchResults() {
    clearTimeout(Timer);
    const cardsContainer = document.getElementById("CardsContainer");
    const searchQuery = document.getElementById('gameSearch').value.toLowerCase();

    // skeleton animation
    if (!justUpdated) {
        justUpdated = true;
        cardsContainer.querySelectorAll(".gameCardS, .gameCard, .gameCardE").forEach(container => {
            const skeletonDiv = document.createElement('a');

            container.classList.forEach(name => {
                skeletonDiv.classList.add(name);
            });

            skeletonDiv.classList.add("skeletonLoading");
            cardsContainer.replaceChild(skeletonDiv, container);
        });
    }

    Timer = setTimeout(() => {
        let jsonData = [];
        let isCodeSearch = codeRegex.test(searchQuery);

        databaseJsonData.forEach(game => {
            // id based searching
            if (isCodeSearch && (game.code.toLowerCase() !== searchQuery)) {
                return;

                // title based searching
            } else if (!isCodeSearch && !game.title.toLowerCase().includes(searchQuery)) {
                return;
            }

            // filter tags
            if (tagFilter.length > 0) {
                let isGood = false;

                for (const tag of tagFilter) {
                    if (tag === game.tag) {
                        isGood = true;
                        break;
                    }
                }

                if (isGood === false) {
                    return;
                }
            }

            jsonData.push(game);
        });


        let startSlice = (currentPage - 1) * issuesPerPage; // makes it start on 0
        let endSlice = startSlice + issuesPerPage;

        totalPages = Math.ceil(jsonData.length / issuesPerPage);
        issuesCount = jsonData.length;

        gameCardHandler(jsonData.slice(startSlice, endSlice));
        PageSelectorUpdater();
        justUpdated = false;

    }, 300);
}


// Game Card handler, it updates the game cards
function gameCardHandler(jsonData) {
    const gameWrapper = document.getElementById("CardsContainer");
    gameWrapper.innerHTML = "";


    let currentIssue = 0;
    let totalCurrentIssues = jsonData.length;

    jsonData.forEach(game => {
        currentIssue++;

        let imageSource;
        let cardType;
        let imageText = "N/A";
        let imageTextSize = 1.25;

        // sets the image url's && text in case there is no image
        switch (true) {
            case game.image && avifSupport && game.type === "HB":
                imageSource = "./_images/hb/" + game.title + ".avif";
                break;
            case game.image && avifSupport && game.type === "GAME":
                imageSource = "./_images/games/" + game.code + ".avif";
                break;

            case (!game.image || !avifSupport) && game.type === "HB":
                imageText = "HOME<br>BREW";
                break;

            case (!game.image || !avifSupport) && game.type === "GAME":
                imageText = "GAME";
                break;
        }

        // makes the game cards have the correct "form" (aka rounding of the edges)
        switch (true) {
            case totalCurrentIssues === 1 :
                cardType = "gameCard";
                break;
            case currentIssue === 1 :
                cardType = "gameCardS";
                break;
            case currentIssue === totalCurrentIssues :
                cardType = "gameCardE";
                break;
        }

        let last_updated = new Date(game.updated).toLocaleDateString();

        // game cards html
        const gameElementHTML = `
            <a class="${cardType ? cardType : `gameCard`} ${game.tag}" target="_blank" rel="noopener" href="https://github.com/obhq/compatibility/issues/${game.id}">
                ${imageSource ? `<img class="gameCardImage" loading="lazy" alt="${game.title} - ${game.code} game image" src="${imageSource}">` : `<p class='gameCardImageText' style='font-size: ${imageTextSize}rem;'>${imageText}</p>`}
                <div class="gameContent">
                    <p class="gameCardTitle">${game.title}</p>
                    <p class="gameCardCode">${game.code}</p>
                    <p class="gameCardTag">${game.tag}</p>
                    <p class="gameCardUpdated">${last_updated}</p>
                </div>
            </a>`;

        const tempContainer = document.createElement('div');
        tempContainer.innerHTML = gameElementHTML;

        gameWrapper.appendChild(tempContainer.firstElementChild);
    });

    // updates the results found text on the bottom
    if (totalCurrentIssues === 0) {
        document.getElementById("infoText").innerText = `No results found!`;

        // adds the NoResultFound message. Also, why the fuck do I need to put two "\", like why??? I NEED ANSWERS
        gameWrapper.innerHTML = `
        <div class="noResultsFound">
            <span class="noResultsFoundText">No Results found</span>
            <span class="noResultsFoundEmoji">¯\\_(ツ)_/¯</span>
        </div>`;
        
    } else {
        document.getElementById("infoText").innerText = `${issuesCount} results found`;
    }

    // Fixes the page jumping when the last page doesn't have 10 issues
    if (currentPage === (totalPages - 1)) {
        window.scrollTo({behavior: 'instant', top: document.body.scrollHeight});
    }
}


// updates the values of the page selector and handles the changing of the page number
function PageSelectorUpdater(state) {
    const minNumberElement = document.getElementById("pageSelectorMin");
    const maxNumberElement = document.getElementById("pageSelectorMax");
    const pageSearchElement = document.getElementById("pageSelectorSearch");

    let oldPage = currentPage;

    switch (state) {
        case "search":
            if (!isNaN(parseInt(pageSearchElement.value))) {
                currentPage = parseInt(pageSearchElement.value);
            }

            pageSearchElement.value = "";
            break;
        case "min":
            currentPage = 1;
            break;
        case "max":
            currentPage = totalPages;
            break;
        case "less":
            currentPage--;
            break;
        case "more":
            currentPage++;

            break;
    }

    if (currentPage <= 1) {
        currentPage = 1;
        pageSearchElement.placeholder = "...";
        minNumberElement.classList.add("pageBarSelected");
        maxNumberElement.classList.remove("pageBarSelected");
        pageSearchElement.classList.remove("pageBarSelected");

    } else if (currentPage >= totalPages) {
        currentPage = totalPages;
        maxNumberElement.classList.add("pageBarSelected");
        minNumberElement.classList.remove("pageBarSelected");
        pageSearchElement.classList.remove("pageBarSelected");
        pageSearchElement.placeholder = "...";

    } else {
        minNumberElement.classList.remove("pageBarSelected");
        maxNumberElement.classList.remove("pageBarSelected");
        pageSearchElement.classList.add("pageBarSelected");
        pageSearchElement.placeholder = currentPage;
    }

    if (!(currentPage === oldPage)) {
        updateSearchResults();
    }

    // if there are no search results, set the minNumberElement to 0
    if (totalPages === 0) {
        minNumberElement.innerText = "0";
    } else {
        minNumberElement.innerText = "1";
    }

    maxNumberElement.innerText = totalPages;
}


// gets called when the onInput is called on the page selector "search bar"
function OnPageSelectorSearch() {
    clearTimeout(Timer);

    Timer = setTimeout(() => {

        PageSelectorUpdater("search");

    }, 700);
}