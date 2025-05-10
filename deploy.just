# In order to make just recipes work - you will need to
# have those ENV variables in your .env file
set dotenv-load

# The default username ofc is not correct
USERNAME := env_var_or_default("DEPLOY_USERNAME", "hans")
DOMAIN := env_var_or_default("DEPLOY_DOMAIN", "digidog.rust-basel.ch")
NAME := env_var_or_default("DEPLOY_NAME", "digidog-server")
LETS_ENCRYPT_DOMAN:= env_var_or_default("LETSENCRYPT_DOMAIN", "digidog.rust-basel.ch")
ARTIFACT_PATH:=env_var_or_default("ARTIFACT_PATH", "target/digidog-0.0.1-ALPHA.jar")
VIDEO_PATH:=env_var_or_default("VIDEO_PATH", "video.mp4")

ssh:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }}

create_image:
    podman build -t build_image -f dockerfiles/Containerfile .

build:
    podman run --rm -v $(pwd):/project \
    -w /project \
    build_image:latest \
    cargo build --release --target aarch64-unknown-linux-gnu

build_all:
    just build digidog-server

deploy name:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }} "mkdir -p {{ NAME }}"
    rsync -avz -e "ssh -i ~/.ssh/{{ NAME }}" dist/ {{ USERNAME }}@{{ DOMAIN }}:~/{{ NAME }}/

start name:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }} "sudo systemctl restart {{ name }}"

restart_nginx:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }} "systemctl restart nginx"

stop name:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }} "sudo systemctl stop {{ name }}"

log name:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }} "sudo systemctl status {{ name }}"

remote command:
    ssh -i ~/.ssh/{{ NAME }} {{ USERNAME }}@{{ DOMAIN }} "{{ command }}"

setup_letsencrypt:
    just remote "sudo apt update"
    just remote "sudo apt install certbot python3-certbot-nginx"
    just remote "sudo certbot --nginx -d {{LETS_ENCRYPT_DOMAN}}"

update_letsencrypt:
    just remote "sudo certbot renew"

setup_nginx:
    just copy_remote "nginx/default.conf" "/etc/nginx/sites-available/default"
    just remote "sudo rm /etc/nginx/sites-enabled/default 2>/dev/null"
    just remote "sudo ln -s /etc/nginx/sites-available/default /etc/nginx/sites-enabled/"

copy_remote source_host dest_remote:
    rsync -avz -e "ssh -i ~/.ssh/{{ NAME }}" {{ source_host }} {{ USERNAME }}@{{ DOMAIN }}:{{ dest_remote }}

deploy_server:
    just build
    just copy_remote "target/aarch64-unknown-linux-gnu/release/{{NAME}}" "~/{{NAME}}"

deploy_dist:
    just remote "mkdir -p jars"
    just copy_remote "{{ARTIFACT_PATH}}" "~/jars/$(basename {{ARTIFACT_PATH}})"
    just copy_remote "digi.md" "~/digi.md"
    just copy_remote "{{VIDEO_PATH}}" "~/$(basename {{VIDEO_PATH}})"

setup_systemd:
    just copy_remote "systemd/{{NAME}}.service" "/etc/systemd/system/{{NAME}}.service"
    just remote "sudo systemctl daemon-reload"
    just remote "sudo systemctl enable {{NAME}}"
    just remote "sudo systemctl start {{NAME}}"

remote_copy_env:
    just copy_remote ".env" "~/.env"

setup_all:
    just setup_letsencrypt
    just setup_nginx
    just setup_systemd

deploy_all:
    just remote_copy_env
    just deploy_server
    just deploy_dist

stop_all:
    just stop {{NAME}}
    just stop nginx

start_all:
    just start {{NAME}}
    just start nginx