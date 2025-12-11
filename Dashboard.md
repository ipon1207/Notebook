# 🚀 Dashboard

> [!INFO] WELCOME BACK!
> 今日も1日、積み上げよう

---

## 📅 Habits & Progress

```dataviewjs
// --- 設定開始 ---
const folderName = '"Daily Notes"'; // 日記のフォルダ名（ダブルクォートで囲む）
const targetYear = 2025;            // 表示したい年
// --- 設定終了 ---

dv.span("**📚 Study Log**");

const calendarData = {
    year: targetYear,
    colors: {
        0: ["#f2f2f2", "#f2f2f2", "#f2f2f2", "#f2f2f2", "#f2f2f2"],
        1: ["#ffdfd9", "#ffbfb3", "#ff9f8e", "#ff7f68", "#ff5f43"],
    },
    entries: []
};

// 指定フォルダから、study_time（勉強時間）が記録されているノートを取得
const pages = dv.pages(folderName).where(p => p.study_time);

for (let page of pages) {
    calendarData.entries.push({
        date: page.file.name, // ノート名が "2025-01-01" のような形式である必要があります
        intensity: page.study_time,
        content: "" // マスの中に文字を表示したい場合はここに "✓" などを入れる
    });
}

// カレンダーを描画
renderHeatmapCalendar(this.container, calendarData);
```

---

## 🔥 Today's Tasks

> [!IMPORTANT] 優先タスク
> 今日が期限、期限切れのタスク

```Tasks
not done
due before or on today
sort by priority
hide backlink
```

---

## 🕰️ Current Projects / Learning

> [!example] 進行中の勉強・プロジェクト
> タグ　`#project/active` または `#study/now` が付いているノート

```Dataview
TABLE without id file.link as "Project", file.mday as "Last Modified"
FROM #project/active OR #study/now
SORT file.mday DESC
LIMIT 5
```

---

## 📝 Recent Notes

> [!quote] 最近のインプット
> 最近作成した3日分のノート（Daily Noteを除く）

```Dataview
LIST
FROM ""
WHERE file.cday >= date(today) - dur(3 days)
AND !contains(file.folder, "Daily Notes")
SORT file.ctime DESC
```

