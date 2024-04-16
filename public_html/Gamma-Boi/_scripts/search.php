<?php
$startTime = microtime(true); // timer

/// connecting to database
try {
    $db = new PDO("sqlite:/mnt/SnowData/snowy/Documents/Git/Obliteration.net/updater/main.db");  //todo : change
//    $db = new PDO("sqlite:G:\OBHQdb\main.db");  //todo : change
} catch (PDOException) {
    die("The server got itself into trouble, please try to refresh.<br>");
}

/// processing user input
$pageReq = $_GET['page'] ?? 1;
$oldestReq = isset($_GET['oldest']);
$statsReq = isset($_GET['stats']);

$searchQuery = htmlspecialchars($_GET['q'] ?? "");
$tags = htmlspecialchars($_GET['tag'] ?? "");
$page = filter_var($pageReq, FILTER_VALIDATE_INT) ? $pageReq : 1;
$oldest = filter_var($oldestReq, FILTER_VALIDATE_BOOLEAN) ? $oldestReq : false;
$giveStats = filter_var($statsReq, FILTER_VALIDATE_BOOLEAN) ? $statsReq : false;

if ($page < 0 || $page === 0) {
    $page = 1;
}

//SELECT * FROM issues WHERE code LIKE :searchQuery ORDER BY id DESC LIMIT :offset, :limit

/// defining stuff
$maxResults = (!empty($searchQuery)) ? 10 : 20;
$pageNumber = ($page - 1) * $maxResults;
$sqlQuery = "SELECT * FROM issues";
$usedWhere = false;
$params = [];

if ($searchQuery) {
    $searchQuery = strtolower($searchQuery);

    if (preg_match('/[a-zA-Z]{4}[0-9]{5}/', $searchQuery) & strlen($searchQuery) <= 9) {
        $sqlQuery .= " WHERE code LIKE :searchQuery";
    } else {
        $sqlQuery .= " WHERE title LIKE :searchQuery";
    }

    $params[] = ($searchQuery . '%');
}

if ($tags) {
    $tagFilters = explode(',', $tags);

    $sqlQuery .= ($searchQuery ? " AND" : " WHERE") . " tag IN (?";

    $params[] = $tagFilters[0];

    for ($i = 1; $i < count($tagFilters); $i++) {
        $sqlQuery .= ",?";
        $params[] = $tagFilters[$i];
    }

    $sqlQuery .= ")";
}


/// gets the total amount of issues based on the search query
$sqlQueryForTotal = "SELECT COUNT(*) FROM ($sqlQuery)";
$stmtTotal = $db->prepare($sqlQueryForTotal);

for ($i = 0; $i < count($params); $i++) {
    $stmtTotal->bindValue(($i + 1), $params[$i]);
}

$stmtTotal->execute();
$totalIssuesAmount = $stmtTotal->fetchColumn();
$totalPages = ceil($totalIssuesAmount / $maxResults);


/// forming and executing sql query
$sqlQuery .= " ORDER BY id " . ($oldest ? "ASC" : "DESC");
$sqlQuery .= " LIMIT :offset, :limit";
$params[] = $pageNumber;
$params[] = $maxResults;

$stmt = $db->prepare($sqlQuery);

for ($i = 0; $i < count($params); $i++) {
    $stmt->bindValue(($i + 1), $params[$i]);
}

$stmt->execute();
$result = $stmt->fetchAll(PDO::FETCH_ASSOC);

/// Outputting results
header('Content-Type: application/json');

$games = array();
$stats = array();

foreach ($result as $game) {
    $games[] = array(
        "id" => $game['id'],
        "title" => $game['title'],
        "code" => $game['code'],
        "type" => $game['type'],
        "tag" => $game['tag'],
        "upDate" => $game['updatedDate'],
        "image" => (bool)$game['image']
    );
}

/// stats on the compatibility list
if ($giveStats === true) {
    $availableTags = ['Nothing', 'Boots', 'Intro', 'Ingame', 'Playable'];
    $naTag = "N/A";
    $tagPercentages = [];

    $stmt = $db->prepare("SELECT COUNT(*) FROM issues WHERE tag = :tag");
    $stmt->bindParam(':tag', $naTag);
    $stmt->execute();
    $naCount = $stmt->fetchColumn();

    $result = $db->query("SELECT COUNT(*) FROM issues");
    $total = $result->fetchColumn();

    $issuesWithoutNA = $total - $naCount;

    foreach ($availableTags as $tag) {
        $stmt = $db->prepare("SELECT COUNT(*) FROM issues WHERE tag = :tag");
        $stmt->bindParam(':tag', $tag);
        $stmt->execute();

        $count = $stmt->fetchColumn();
        $percentage = ($issuesWithoutNA > 0) ? ($count / $issuesWithoutNA) * 100 : 0;
        $tagPercentages[$tag] = $percentage;
        $tagCount[$tag] = $count;
    }

    $amount = (100 - array_sum($tagPercentages)) / count($tagPercentages);

    foreach ($availableTags as $tag) {
        $percentage = $tagPercentages[$tag];
        $count = $tagCount[$tag];
        $percentage += $amount;
        $percentage = round($percentage, 2);
        $stats[] = array(
            "tag" => $tag,
            "percent" => $percentage,
            "count" => $count
        );
    }
}

$executionTime = round((microtime(true) - $startTime) * 1000, 2); // Convert to milliseconds

$info = array(
    "issues" => $totalIssuesAmount,
    "pages" => $totalPages,
    "time" => $executionTime
);

$data = array(
    "info" => $info,
    "games" => $games,
    "stats" => $stats,
);


/// end
$jsonData = json_encode($data);
echo $jsonData;
$db = null; //exit connection
?>