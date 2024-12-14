class Widget {
    constructor() {
        this.ready = false; // @todo: Add initialization
    }

    // @bug: Sometimes fails on Safari
    render() {
        return '<div>Widget</div>';
    }
}
