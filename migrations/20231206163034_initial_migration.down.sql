DROP TABLE IF EXISTS AppUser, Game, GameMatch, Odds, Bet,
                     MoneyTransaction CASCADE;

DROP TYPE IF EXISTS GameGenre, GameMatchStatus, GameMatchOutcome,
                    BetStatus, Currency, MoneyTransactionStatus;