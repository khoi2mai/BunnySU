#  🐰 BunnySU

<p align="center">
  <img src="https://raw.githubusercontent.com/khoi2mai/BunnySU/refs/heads/bunnysu/docs/assets/bunnysu_logo.png" width="96" alt="BunnySU Logo">
</p>

<p align="center">
  <strong>פתרון root מודרני מבוסס kernel עבור מכשירי Android.</strong>
</p>

<p align="center">
  <a href="https://github.com/khoi2mai/BunnySU/releases/latest">
    <img src="https://img.shields.io/github/v/release/khoi2mai/BunnySU?label=Release&logo=github" alt="Latest Release">
  </a>
  <a href="/LICENSE">
    <img src="https://img.shields.io/github/license/khoi2mai/BunnySU?logo=gnu" alt="License">
  </a>
  <a href="https://t.me/KernelSU">
    <img src="https://img.shields.io/badge/Telegram-Community-blue?logo=telegram" alt="Telegram">
  </a>
</p>

<p align="center">
  <a href="README.md">English</a> |
  <a href="README_ES.md">Español</a> |
  <a href="README_CN.md">简体中文</a> |
  <a href="README_TW.md">繁體中文</a> |
  <a href="README_JP.md">日本語</a> |
  <a href="README_KR.md">한국어</a> |
  <a href="README_PL.md">Polski</a> |
  <a href="README_PT-BR.md">Português</a> |
  <a href="README_TR.md">Türkçe</a> |
  <a href="README_RU.md">Русский</a> |
  <a href="README_VI.md">Tiếng Việt</a> |
  <a href="README_ID.md">Indonesia</a> |
  <a href="README_IN.md">हिन्दी</a> |
  <a href="README_IT.md">Italiano</a> |
  <a href="README_IW.md">עברית</a>
</p>

---

## סקירה כללית

BunnySU הוא פתרון root ברמת kernel המיועד למכשירי Android.
הוא מספק ניהול חזק של הרשאות root ישירות משכבת ה-kernel, ומציע חוויה נקייה, יעילה ו-systemless עבור משתמשים מתקדמים, מפתחים וחובבי Android.

בשונה מפתרונות root מסורתיים המבוססים רק על userspace, BunnySU משתלב עם ה-kernel כדי לספק שליטה עמוקה יותר, בידוד חזק יותר וניהול הרשאות גמיש יותר.

---

## תכונות

* **גישת root מבוססת kernel**
  מספק תמיכה ב-`su` ישירות דרך ה-kernel של Android.

* **ניהול הרשאות root**
  שליטה באילו אפליקציות יכולות לגשת ל-root באמצעות מערכת הרשאות ברורה ונוחה לניהול.

* **תמיכה במודולים systemless**
  תומך בשינויים מבוססי מודולים בלי לשנות ישירות את מחיצת המערכת.

* **שליטת App Profile**
  הגבלה, בידוד או התאמה אישית של התנהגות root עבור אפליקציות בודדות.

* **תמיכה ב-GKI**
  מיועד למכשירי Android מודרניים המשתמשים בארכיטקטורת Generic Kernel Image.

* **תאימות מתקדמת**
  תומך בטלפוני Android,‏ WSA,‏ ChromeOS וסביבות Android מבוססות container.

---

## תאימות

BunnySU מיועד רשמית למכשירי Android המשתמשים ב-**GKI 2.0** עם kernel בגרסה **5.10 ומעלה**.

גם kernels ישנים יותר עשויים לעבוד, אך בדרך כלל הם דורשים שילוב ידני ב-kernel ובניות ייעודיות למכשיר.

### ארכיטקטורות נתמכות

* `arm64-v8a`
* `x86_64`

> [!CAUTION]
> חלק משינויי ה-kernel האחרונים עלולים להשפיע על תאימות במכשירי `x86_64` מסוימים ולגרום לבעיות אתחול או kernel panic.
> תמיד בדוק מידע ספציפי למכשיר לפני צריבה או בנייה.

---

## התקנה

יש לקרוא את מדריך ההתקנה הרשמי לפני השימוש ב-BunnySU.

* [Installation Guide](https://kernelsu.org/guide/installation.html)
* [How to Build](https://kernelsu.org/guide/how-to-build.html)
* [Official Website](https://kernelsu.org/)

> [!WARNING]
> ביצוע root ושינוי ה-kernel עלולים לגרום לכשל אתחול, אובדן נתונים או חוסר יציבות במכשיר אם הם נעשים בצורה שגויה.
> ודא שאתה מבין את הסיכונים ושמור גיבוי מלא לפני שתמשיך.

---

## בנייה

ניתן לשלב את BunnySU ב-kernels נתמכים של Android ולבנות אותו ידנית.

להוראות בנייה מפורטות, עיין ב:

* [Build Documentation](https://kernelsu.org/guide/how-to-build.html)

מתחזקים של מכשירים צריכים לוודא שקוד המקור של ה-kernel, קובץ ה-defconfig וסביבת הבנייה מוגדרים כראוי לפני השילוב.

---

## מערכת מודולים

BunnySU תומך במערכת מודולים systemless המבוססת על metamodules.

ניתן להשתמש במודולים כדי לשנות את התנהגות המערכת, להוסיף יכולות או להתאים אישית את סביבת Android בלי לשנות ישירות את מחיצת המערכת.

למידע נוסף:

* [Metamodule Documentation](https://kernelsu.org/guide/metamodule.html)

---

## App Profile

App Profile מאפשר שליטה מדויקת בהרשאות root ובהתנהגות אפליקציות.

באמצעות App Profile, משתמשים יכולים להגביל גישת root, להתאים הרשאות ולהפחית חשיפה מיותרת של יכולות root רגישות.

למידע נוסף:

* [App Profile Documentation](https://kernelsu.org/guide/app-profile.html)

---

## אבטחה

BunnySU פועל ברמה נמוכה של מערכת Android.
אנא דווח על בעיות אבטחה בצורה אחראית והימנע מפרסום פומבי של חולשות לפני שנבדקו.

למידע הקשור לאבטחה, ראה:

* [Security Policy](SECURITY.md)

---

## רישיון

BunnySU עוקב אחר מבנה הרישוי המקורי של KernelSU:

* קבצים תחת הספרייה `kernel` מורשים תחת **GPL-2.0-only**.
* חלקים אחרים של הפרויקט מורשים תחת **GPL-3.0-or-later**, אלא אם צוין אחרת.

ראה את הקובץ [LICENSE](LICENSE) לפרטים נוספים.

---

## קרדיטים

BunnySU שואב השראה ונבנה על עבודת קהילת ה-root ופיתוח ה-kernel של Android.

תודה מיוחדת ל:

* [Kernel-Assisted Superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/)
* [Magisk](https://github.com/topjohnwu/Magisk)
* [genuine](https://github.com/brevent/genuine/)
* [Diamorphine](https://github.com/m0nad/Diamorphine)

---

<p align="center">
  <strong>BunnySU</strong><br>
  גישת root ברמת kernel עבור מכשירי Android מודרניים.
</p>
