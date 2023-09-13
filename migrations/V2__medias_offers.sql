CREATE TABLE medias_offers (
  media_id UUID NOT NULL REFERENCES medias(media_id),
  offer_id UUID NOT NULL
);