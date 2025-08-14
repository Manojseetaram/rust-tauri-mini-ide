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
