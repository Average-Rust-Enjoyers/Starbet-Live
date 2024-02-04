BEGIN TRANSACTION;

/* Demo user accounts

 Admin account
   e-mail: admin@admin.com
   password: DemoAdmin123!

 User account
   e-mail: demo@user.com
   password: Password123!

*/

INSERT INTO "appuser"
    ("id", "username", "email", "name", "surname", "profile_picture", "password_hash",
     "balance", "created_at", "edited_at", "deleted_at", "is_admin")
VALUES
    ('8fd35312-9642-4dd4-a8f4-bf3e72eb2d9d',
     'demoadmin', 'admin@admin.com', 'admin', 'badmin',
     'https://robohash.org/demoadmin.png?set=set2',
     '$argon2id$v=19$m=19456,t=2,p=1$xDy6blM0/5OApopgPCymvA$JuyCzXHvKPC9VUiahKDi/arZ1w9/P9FjAVD+BE1aFjA',
     4,	now(), now(), NULL,	't'),

    /* e-mail: demo@user.com
       password: Password123! */
    ('27a5dfb0-bf7d-46d8-b880-2acd13879912',
     'demouser1', 'demo@user.com', 'demo', 'user',
     'https://robohash.org/demouser1.png?set=set2',
     '$argon2id$v=19$m=19456,t=2,p=1$icvnlhwoh2Dp8cryxcWKNw$NwzcdekpTN0CXZkCCbe7lF7aDZwnKBudNV/2aGvkMAg',
     0, now(), now(), NULL, 'f');


/* Demo money transactions */
INSERT INTO "moneytransaction" ("id", "app_user_id", "status", "amount_tokens", "amount_currency", "currency", "deposit", "created_at", "edited_at") VALUES
('a496912a-49f4-44c0-9b75-3a8cacced525',	'8fd35312-9642-4dd4-a8f4-bf3e72eb2d9d',	'COMPLETED',	17138,	48.33,	'EUR',	't',	now(), now()),
('b496912a-49f4-44c0-9b75-3a8cacced525',	'8fd35312-9642-4dd4-a8f4-bf3e72eb2d9d',	'COMPLETED',	16649,	24.81,	'CZK',	't',	now(), now()),
('c496912a-49f4-44c0-9b75-3a8cacced525',	'27a5dfb0-bf7d-46d8-b880-2acd13879912',	'COMPLETED',	16649,	24.81,	'CZK',	't',	now(), now()),
('d496912a-49f4-44c0-9b75-3a8cacced525',	'27a5dfb0-bf7d-46d8-b880-2acd13879912',	'COMPLETED',	16649,	24.81,	'CZK',	't',	now(), now());

/* Demo games */
INSERT INTO "game" ("id", "name", "description", "logo", "genre", "created_at", "edited_at", "deleted_at", "cloudbet_key") VALUES
('b1f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'League of Legends',	'League of Legends is a team-based game with over 140 champions to make epic plays with.',	'https://gaming-cdn.com/images/products/9456/orig/league-of-legends-pc-game-cover.jpg?v=1662363312',	'MOBA',	'2023-12-09 19:38:46.728083+00',	'2023-12-09 19:38:46.728083+00',	NULL,	'league-of-legends'),
('02f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Counter-Strike 2',	'Counter-Strike 2 expands upon the team-based action gameplay that it pioneered when it was launched 19 years ago.',	'https://cdn.cloudflare.steamstatic.com/steam/apps/730/header.jpg?t=1607019058',	'FPS',	'2023-12-09 19:38:46.728083+00',	'2023-12-09 19:38:46.728083+00',	NULL,	'counter-strike'),
('03f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Dota 2',	'Dota 2 is a multiplayer online battle arena (MOBA) video game developed and published by Valve.',	'https://cdn.cloudflare.steamstatic.com/steam/apps/570/header.jpg?t=1607022750',	'MOBA',	'2023-12-09 19:38:46.728083+00',	'2023-12-09 19:38:46.728083+00',	NULL,	'dota-2');

/* Demo matches */
INSERT INTO "gamematch" ("id", "game_id", "name_a", "name_b", "starts_at", "ends_at", "status", "outcome", "created_at", "edited_at", "deleted_at", "cloudbet_id") VALUES
('b4f8382a-fcba-42f2-a38c-c7d0044739bb',	'b1f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Example team A - 1',	'TeamB',	'2024-02-02 22:15:00+00',	'2024-02-12 22:15:00+00',	'PENDING',	NULL,	'2024-02-02 00:21:45.911171+00',	'2024-02-03 18:50:03.668491+00',	NULL,	NULL),
('05e6a130-8ab7-4bfb-abfa-cbd54351d946',	'b1f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Example team A - 2',	'TeamB',	'2024-02-02 16:00:00+00',	'2024-02-12 16:00:00+00',	'PENDING',	NULL,	'2024-02-02 00:21:44.353117+00',	'2024-02-03 18:50:01.492513+00',	NULL, NULL),
('06195936-d389-47b6-a2e0-75303c81a148',	'02f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Example team A - 3',	'TeamB',	'2024-02-02 08:15:00+00',	'2024-02-12 08:15:00+00',	'LIVE',	NULL,	'2024-02-02 00:21:42.784343+00',	'2024-02-02 03:59:19.36748+00',	NULL,	NULL),
('b7817b93-4f3a-42ad-8d4f-cbb83cf65241',	'02f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Example team A - 4',	'TeamB',	'2024-02-02 21:00:00+00',	'2024-02-12 21:00:00+00',	'PENDING',	NULL,	'2024-02-02 00:21:43.781911+00',	'2024-02-02 03:59:58.575378+00',	NULL,	NULL),
('18d14143-34ff-4cd5-b711-466a9762ece6',	'03f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Example team A - 5',	'TeamB',	'2024-02-02 10:45:00+00',	'2024-02-12 10:45:00+00',	'PENDING',	NULL,	'2024-02-02 00:21:42.972367+00',	'2024-02-02 03:59:20.250669+00',	NULL,	NULL),
('e911b5d2-e3bc-4ab9-b32c-9c3ee793df05',	'03f1c213-30e5-4cac-9b05-e0a1befbe7ee',	'Example team A - 6',	'TeamB',	'2024-02-02 18:00:00+00',	'2024-02-12 18:00:00+00',	'PENDING',	NULL,	'2024-02-02 00:21:43.17147+00',	'2024-02-02 03:59:21.547492+00',	NULL,	NULL);


/* Demo odds */
INSERT INTO "odds" ("id", "game_match_id", "odds_a", "odds_b", "created_at", "deleted_at") VALUES
('c0faf59a-c34b-4adf-a778-33e5bfec6331',	'b4f8382a-fcba-42f2-a38c-c7d0044739bb',	1.9,	1.9,	'2024-02-02 00:21:41.542758+00',	NULL),
('b0faf59a-c34b-4adf-a778-33e5bfec6332',	'05e6a130-8ab7-4bfb-abfa-cbd54351d946',	1.9,	1.9,	'2024-02-02 00:21:41.542758+00',	NULL),
('c0faf59a-c34b-4adf-a778-33e5bfec6333',	'06195936-d389-47b6-a2e0-75303c81a148',	1.9,	1.9,	'2024-02-02 00:21:41.542758+00',	NULL),
('b0faf59a-c34b-4adf-a778-33e5bfec6334',	'b7817b93-4f3a-42ad-8d4f-cbb83cf65241',	1.9,	1.9,	'2024-02-02 00:21:41.542758+00',	NULL),
('c0faf59a-c34b-4adf-a778-33e5bfec6335',	'18d14143-34ff-4cd5-b711-466a9762ece6',	1.9,	1.9,	'2024-02-02 00:21:41.542758+00',	NULL),
('b0faf59a-c34b-4adf-a778-33e5bfec6336',	'e911b5d2-e3bc-4ab9-b32c-9c3ee793df05',	1.9,	1.9,	'2024-02-02 00:21:41.542758+00',	NULL);

COMMIT;
