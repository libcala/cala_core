var _cala_stack = new Array();
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
    let tmp = _cala_js_malloc(str);
    return tmp;
}
function _cala_js_function(i) {
    let offset = _cala_stack.length;
    _cala_stack.push(Function(_cala_heap[i])());
    return offset;
}
function _cala_js_call(f, a, b) { return _cala_stack[f](_cala_heap[a], _cala_heap[b]); }
function _cala_js_free(i) { return _cala_garbage.push(i); }
