# Flashcards
### Aplikace pro tvorbu a procvičování kartiček pro pomoc s učením.
Projekt se skládá z backendu (Axum) a frontendu (Yew).

## Spuštění projektu (Windows)
`./run-be_fe.bat`

## Struktura projektu
- `api/` - Zdrojový kód backendu (Axum)
- `client/` - Zdrojový kód frontendu (Yew)
- `api/tests` - Testy

## Struktura backendu (api/src)
- `main.rs` - Hlavní soubor aplikace
- `database.rs` - Připojení k databázi a vytváření tabulek
- `lib.rs` - Vstupní bod knihovny, zpřístupňuje moduly aplikace pro testy
- `dto/` - Data Transfer Objects pro komunikaci mezi klientem a serverem
- `routes/` - Definice jednotlivých rout
- `utils/` - Pomocné funkce a utility

## Struktura frontendu (client/src)
- `main.rs` - Hlavní soubor aplikace
- `pages/` - Jednotlivé stránky aplikace
- `components/` - Komponenty
- `auth/` - Logika autentikace
- `utils/` - Pomocné funkce a utility
- `utils/hoos/` - Vlastní hooky (především pro načítání dat z API)
- `utils/types/` - structury