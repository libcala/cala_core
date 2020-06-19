var _cala_heap = new Array();
var _cala_garbage = new Array();
function _cala_js_malloc(o) {
    if(_cala_garbage.length != 0) {
        let idx = _cala_garbage.pop();
        _cala_heap[idx] = o;
        return idx;
    } else {
        let idx = _cala_heap.length;
        _cala_heap.push(o);
        return idx;
    }
}
function _cala_js_string(p,l) {
    var buf = new Uint16Array(Module.instance.exports.memory.buffer,p,l);
    var str = "";
    for(var i = 0; i < l; i++) {
        str += String.fromCharCode(buf[i]);
    }
    return _cala_js_malloc(str);
}
function _cala_js_function(i) {
    return _cala_js_malloc(Function(_cala_heap[i])());
}
function _cala_js_call(f, a, b) {
    let o = _cala_heap[f](_cala_heap[a], _cala_heap[b]);
    if(o == undefined) {
        return 4294967295;
    } else {
        return _cala_js_malloc(o);
    }
}
function _cala_js_free(i) { return _cala_garbage.push(i); }
function _cala_js_readstr(j, p, l) {
    var buf = new Uint16Array(Module.instance.exports.memory.buffer,p,l);
    let get = _cala_heap[j];
    for(var i = 0; i < l; i++) {
        buf[i] = get.charCodeAt(i);
    }
    return get.length;
}
function _cala_js_read8(j, p, l) {
    var buf = new Uint8Array(Module.instance.exports.memory.buffer,p,l);
    let get = _cala_heap[j];
    for(var i = 0; i < l; i++) {
        buf[i] = get[i];
    }
    return get.length;
}
function _cala_js_read16(j, p, l) {
    var buf = new Uint16Array(Module.instance.exports.memory.buffer,p,l);
    let get = _cala_heap[j];
    for(var i = 0; i < l; i++) {
        buf[i] = get[i];
    }
    return get.length;
}
function _cala_js_read32(j, p, l) {
    var buf = new Uint32Array(Module.instance.exports.memory.buffer,p,l);
    let get = _cala_heap[j];
    for(var i = 0; i < l; i++) {
        buf[i] = get[i];
    }
    return get.length;
}
