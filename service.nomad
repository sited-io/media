job "media" {
  datacenters = ["dc1"]
  type        = "service"

  group "media-api" {
    count = 2

    network {
      mode = "bridge"

      port "grpc" {}
    }

    service {
      name = "media-api"
      port = "grpc"

      connect {
        sidecar_service {
          proxy {
            upstreams {
              destination_name = "zitadel"
              local_bind_port  = 8080
            }
            upstreams {
              destination_name = "cockroach-sql"
              local_bind_port  = 5432
            }
            upstreams {
              destination_name = "commerce-api"
              local_bind_port  = 10000
            }
            upstreams {
              destination_name = "payment-api"
              local_bind_port  = 10001
            }
          }
        }
      }

      check {
        type     = "grpc"
        interval = "20s"
        timeout  = "2s"
      }
    }

    task "media-api" {
      driver = "docker"

      vault {
        policies = ["service-media"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
{{ with nomadVar "nomad/jobs/media" }}
RUST_LOG='{{ .LOG_LEVEL }}'
{{ end }}

HOST='0.0.0.0:{{ env "NOMAD_PORT_grpc" }}'

DB_HOST='{{ env "NOMAD_UPSTREAM_IP_cockroach-sql" }}'
DB_PORT='{{ env "NOMAD_UPSTREAM_PORT_cockroach-sql" }}'
DB_DBNAME='media'
DB_USER='media_user'
{{ with secret "database/static-creds/media_user" }}
DB_PASSWORD='{{ .Data.password }}'
{{ end }}

JWKS_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_zitadel" }}/oauth/v2/keys'
OAUTH_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_zitadel" }}/oauth'
{{ with nomadVar "nomad/jobs/" }}
JWKS_HOST='{{ .JWKS_HOST }}'
OAUTH_HOST='{{ .JWKS_HOST }}'
{{ end }}

{{ with nomadVar "nomad/jobs/media" }}
BUCKET_NAME='{{ .BUCKET_NAME }}'
BUCKET_ENDPOINT='{{ .BUCKET_ENDPOINT }}'
MAX_MESSAGE_SIZE_BYTES='{{ .MAX_MESSAGE_SIZE_BYTES }}'
DEFAULT_USER_QUOTA_MIB='{{ .DEFAULT_USER_QUOTA_MIB }}'
{{ end }}

{{ with secret "kv2/data/services/media" }}
BUCKET_ACCESS_KEY_ID='{{ .Data.data.BUCKET_ACCESS_KEY_ID }}'
BUCKET_SECRET_ACCESS_KEY='{{ .Data.data.BUCKET_SECRET_ACCESS_KEY }}'
SERVICE_USER_CLIENT_ID='{{ .Data.data.SERVICE_USER_CLIENT_ID }}'
SERVICE_USER_CLIENT_SECRET='{{ .Data.data.SERVICE_USER_CLIENT_SECRET }}'
{{ end }}

COMMERCE_SERVICE_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_commerce-api" }}'
PAYMENT_SERVICE_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_payment-api" }}'
EOF
      }

      config {
        image      = "__IMAGE__"
        force_pull = true
      }
    }
  }
}
