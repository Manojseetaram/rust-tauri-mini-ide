//creat a project for frontend

npx create-next-app@latest tauri-ide --typescript

cd ide

//create a project same file directory in backend 


npm install @tauri-apps/api
npm install -D @tauri-apps/cli
npx tauri init


//web browser view 
npm run dev

//desktop view
npx tauri dev


//combined command 

npm run tauri:dev

npm install monaco-editor @monaco-editor/react

// this command is usefull i can start tauri without waching
npm run tauri dev -- --no-watch


//
npm install @tauri-apps/plugin-dialog

//install rust crate
cd src-tauri
cargo add tauri-plugin-dialog
//
npm install @tauri-apps/plugin-fs
//
npm install tailwindcss lucide-react
