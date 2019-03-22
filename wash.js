const OP_SYSTEM = 0;
const SUBOP_INITIALIZATION = 0;
const SUBOP_SPAWN = 1;

class WasmShell extends HTMLElement {
  connectedCallback(){
    let mod = this.getAttribute("module");
    this.autorun = this.getAttribute("autorun");
    this.next_pid = 0;
    this.processes = {};
    this.spawn(mod==null?"wash.wasm":mod);
  }

  spawn(wasmSrc){
    let pid = this.next_pid;
    this.next_pid += 1;
    let component = this;
    this.processes[pid] = WebDOM.run(wasmSrc,{
      global_sys_call:function(op,subOp,paramA,paramB,paramC,paramD){
        if(op == OP_SYSTEM){
          if(subOp == SUBOP_INITIALIZATION){
            if(pid != 0){
              throw new Error("only the first module can initialize");
            }
            let el = this.env.allocator().a(component);
            this.exports.sys_call_handler(0,0,el,pid,0,0);
            if(component.autorun !== null){
              component.spawn(component.autorun)
            }
            return 0
          }
          else if(subOp == SUBOP_SPAWN){
            if(pid != 0){
              throw new Error("only the first module can spawn");
            }
            component.spawn(this.readStringFromMemory(paramA))
            return 0;
          }
          else {
            if(pid != 0 && paramA != 0){
              throw new Error("no support for cross process talk right now")
            } else {
              return component.processes[0].exports.sys_call_handler(op,subOp,pid,paramB,paramC,paramD)
            }
          }
        }
      }
    })
  }
}

customElements.define('wasm-shell', WasmShell);
