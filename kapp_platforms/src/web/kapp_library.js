function get_canvas_size(memory_buffer, output) {
    let canvas = document
        .getElementById("canvas");

    let context = canvas.getContext('2d');
    context.fillRect(0, 0, canvas.width, canvas.height);

    let width = canvas.clientWidth;
    let height = canvas.clientHeight;
    let data = new Float32Array(memory_buffer, output, 2);
    data[0] = width;
    data[1] = height;
    console.log(data);
}

function get_pointer_type(event) {
    switch (event.pointerType) {
        case "mouse": return 1
        case "pen": return 2
        case "touch": return 3
        default:
            return 0
    }
}

var canvas_last_width = 0;
var canvas_last_height = 0;
function request_animation_frame_callback(event) {
    if (canvas.clientWidth != canvas_last_width || canvas_last_height != canvas.clientHeight) { }
    {
        canvas.width = canvas.clientWidth;
        canvas.height = canvas.clientHeight;
        canvas_last_width = canvas.clientWidth;
        canvas_last_height = canvas.clientHeight;
        kwasm_exports.kapp_on_window_resized(canvas.clientWidth, canvas.clientHeight);

    }
    kwasm_exports.kapp_on_animation_frame(kwasm_exports.kapp_on_animation_frame);

}

function pass_f32_to_client(x) {
    let pointer = kwasm_exports.kwasm_reserve_space(4);
    let data_view = new Float32Array(kwasm_memory.buffer, pointer, 4);
    data_view[0] = x;
}

function pass_f32_f32_to_client(x, y) {
    let pointer = kwasm_exports.kwasm_reserve_space(8);
    let data_view = new Float32Array(kwasm_memory.buffer, pointer, 8);
    data_view[0] = x;
    data_view[1] = y;
}

var canvas = document
    .getElementById("canvas");

function receive_message(command, memory_buffer, data, data_length) {
    switch (command) {
        case 0:
            // RequestAnimationFrame
            // Request an animation frame
            request_animation_frame_client_callback = data;
            window.requestAnimationFrame(request_animation_frame_callback)
            break;
        case 1:
            // GetCanvasSize
            // Get the canvas size and write it to data.
            get_canvas_size(memory_buffer, data);
            break;
        case 2:
            // SetCallbacks

            // Hook up callbacks
            canvas.onpointermove = function (event) {
                let pointer_type = get_pointer_type(event);
                kwasm_exports.kapp_on_pointer_move(event.clientX, event.clientY, pointer_type, event.timeStamp);
            }

            canvas.onmousemove = function (event) {
                kwasm_exports.kapp_on_mouse_move(event.movementX, event.movementY, event.timeStamp);
            }

            canvas.onpointerdown = function (event) {
                let pointer_type = get_pointer_type(event);
                kwasm_exports.kapp_on_pointer_down(event.clientX, event.clientY, pointer_type, event.button, event.timeStamp);
            }

            canvas.onpointerup = function (event) {
                let pointer_type = get_pointer_type(event);
                kwasm_exports.kapp_on_pointer_up(event.clientX, event.clientY, pointer_type, event.button, event.timeStamp);
            }

            document.onkeydown = function (event) {
                kwasm_helpers.pass_string_to_client(event.code);
                if (event.repeat) {
                    kwasm_exports.kapp_on_key_repeat(event.timeStamp);
                } else {
                    kwasm_exports.kapp_on_key_down(event.timeStamp);
                }

                // Perhaps these character received events should only be sent if text input has been enabled.

                // Ignore keys pressed while composing an IME character.
                // Also ignore keys that are longer than 1 character.
                // This is incorrect for some non-English key combos, but is an OK heuristic for now
                // to reject non-textual character inputs.
                // A more robust solution may watch a text field for changes instead.
                if (!event.is_composing && event.key.length == 1) {
                    kwasm_helpers.pass_string_to_client(event.key);
                    kwasm_exports.kapp_character_received(event.timeStamp);
                }
            }

            document.onkeyup = function (event) {
                kwasm_helpers.pass_string_to_client(event.code);
                kwasm_exports.kapp_on_key_up(event.timeStamp);
            }

            canvas.on_wheel_callback = function (event) {
                if (event.ctrl_key) {
                    // This is a bit weird, but if a pinch gesture is performed
                    // the ctrl modifier is set.
                    // This is the simplest way to disambiguate it.

                    // 0.02 is a completely arbitrary number to make this value more similar
                    // to what native MacOS produces.
                    // Is this a good idea at all?
                    // Should this library even make such adjustments?
                    // Is there a way to find an actual scale factor instead of a guess?

                    kwasm_exports.kapp_on_pinch(-event.deltaY * 0.02, event.timeStamp);
                } else {
                    kwasm_exports.kapp_on_scroll(-event.deltaX, -event.deltaY, event.timeStamp);
                }
            }
            break;
        case 3:
            // GetDevicePixelRatio
            // This will be sent to Rust as an integer.
            // So this will be incorrect if non-integer values are expected here.
            pass_f32_to_client(window.devicePixelRatio);
            break;
        case 4:
            // GetWindowSize

            let canvas_client_width = canvas.clientWidth;
            let canvas_client_height = canvas.clientHeight;
            // This will be sent to Rust as an integer.
            // So this will be incorrect if non-integer values are expected here.
            pass_f32_f32_to_client(canvas_client_width, canvas_client_height);
            break;
        case 5:
            // LockCursor
            canvas.requestPointerLock();
            break;
        case 6:
            // UnlockCursor
            document.exitPointerLock();
            break;
    }
    return 0;
}

return receive_message;