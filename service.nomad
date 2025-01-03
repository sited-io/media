job "media" {
  datacenters = ["dc1"]
  type        = "service"

  group "media-api" {
    count = 1

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
              destination_name = "nats"
              local_bind_port = 4222
            }
            upstreams {
              destination_name = "postgres-sql"
              local_bind_port  = 5432
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

      resources {
        cpu        = 100
        memory     = 256
        memory_max = 256
      }

      vault {
        policies = ["service-media"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/database_root_cert.crt"
        env         = false 
        change_mode = "restart"
        data        = <<EOF
{{- with secret "kv2/data/services" -}}
{{ .Data.data.DATABASE_ROOT_CERT }}
{{- end -}}
EOF
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
{{ with nomadVar "nomad/jobs/media" }}
RUST_LOG='{{ .RUST_LOG }}'
{{ end }}

HOST='0.0.0.0:{{ env "NOMAD_PORT_grpc" }}'

NATS_HOST='{{ env "NOMAD_UPSTREAM_ADDR_nats" }}'
NATS_USER='{{- with nomadVar "nomad/jobs" -}}{{ .NATS_USER }}{{- end -}}'
NATS_PASSWORD='{{- with secret "kv2/data/services" -}}{{ .Data.data.NATS_PASSWORD }}{{- end -}}'

DB_HOST='{{ env "NOMAD_UPSTREAM_IP_postgres-sql" }}'
DB_PORT='{{ env "NOMAD_UPSTREAM_PORT_postgres-sql" }}'
DB_DBNAME='media'
DB_USER='media_user'
DB_PASSWORD='{{- with secret "database/static-creds/media_user" -}}{{ .Data.password }}{{- end -}}'

{{ with nomadVar "nomad/jobs/" }}
JWKS_URL='http://{{ .JWKS_HOST }}/oauth/v2/keys'
OAUTH_URL='http://{{ .JWKS_HOST }}/oauth'
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
