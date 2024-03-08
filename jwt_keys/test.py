import jwt
import time
import webbrowser

private_key = open('./grafana.key', 'r').read()

payload = {
    "sub": "yolo2",
    "iat": int(time.time())
}

token = jwt.encode(payload, private_key, algorithm='RS256')

base_url = 'http://localhost:3000'
url_with_token = f'{base_url}?auth_token={token}'
print(url_with_token)
# webbrowser.open_new_tab(url_with_token)