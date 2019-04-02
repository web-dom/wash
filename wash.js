const OP_SYSTEM = 0;
const SUBOP_INITIALIZATION = 0;
const SUBOP_SPAWN = 1;

class WasmShell extends HTMLElement {
  connectedCallback(){
    this.nextProcessID = 0;
    let mod = this.getAttribute("module");
    this.autorun = this.getAttribute("autorun");
    this.spawn(mod==null?"wash.wasm":mod,this.nextProcessID);
  }

  spawn(wasmSrc,processId,os){
    let isOperatingSystem = processId == 0;
    let component = this;
    component.nextProcessID+=1;
    WebDOM.run(wasmSrc,{
      joss_syscall:function(request){
        let _request = this.readStringFromMemory(request);
        let command = JSON.parse(_request);
        if(command.operation == "wash_register_os"){
          if(isOperatingSystem){
            let el = this.env.allocator().a(component);
            let response = {
              operation:"wash_os_registered",
              root_component: el
            };
            this.exports.joss_syscall_handler(this.makeString(JSON.stringify(response)));
          }
        }
        else if(command.operation == "wash_spawn_process"){
          if(isOperatingSystem){
              window.setTimeout(()=>{
                  component.spawn(command.path,component.nextProcessID,this);
              },1)
          }
        } else {
          let response = JSON.stringify({operation:"wash_process_command",command:command});
          let r = os.exports.joss_syscall_handler(os.makeString(response));
          let s = os.readStringFromMemory(r);
          return this.makeString(s);
        }
      }
    },function(){})
  }
}

customElements.define('wasm-shell', WasmShell);
