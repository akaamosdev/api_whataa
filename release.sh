#!/bin/bash

echo "🔨 Compilation..."
cargo build --release --target x86_64-pc-windows-gnu

echo "📦 Création du package..."
rm -rf dist/Whataa
mkdir -p dist/Whataa

cp target/x86_64-pc-windows-gnu/release/api_whataa.exe dist/Whataa/


cat > dist/Whataa/run.bat << 'EOF'
@echo off
cd /d "%~dp0"
echo Demarrage API Whataa...
echo.
api_whataa.exe
echo.
echo L'API s'est fermee (erreur ou arret).
echo Consultez: logs\whataa.log et logs\fatal.txt
pause
EOF

cat > dist/Whataa/.env << 'EOF'
DATABASE_URL=postgres://macbookpro:Amos@localhost:5432/whataa_db
EOF

cat > dist/Whataa/README.txt << 'EOF'
WHATAA API

EOF

cd dist
zip -r Whataa.zip Whataa/
cd ..

echo "✅ Prêt! Fichier: dist/Whataa.zip"