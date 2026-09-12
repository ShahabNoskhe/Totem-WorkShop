@echo off
echo Starting Totem Workshop Web Server...
start http://localhost:8080
python -m http.server 8080 --directory dist
