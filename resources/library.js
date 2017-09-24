function adapt_item(item) {
    let close = item.find(".close");
    close.on("click", () => {
        item.remove();
    });

    let header = item.find(".card-header");
    header.append(close);
    header.on("click", () => {
        item.find(".card-body").collapse("toggle");
    });

    return item;
}

function adapt_form(form) {
    form.on("submit", (e) => {
        $.ajax({
            url: form.attr("action"),
            method: form.attr("method"),
            data: form.serialize()
        })
        form.trigger("reset");
        e.preventDefault();
    });
    
    return form;
}

function add_item(id, build) {
    var item = $("#item-" + id);
    if(item.length == 0) { 
        item = $("<div class=\"collapse\" />");
        build(item);
        return item;
    } else {
        return item;
    }
}

$(() => {
    let sheet = $("#sheet");
    $("#add_talent").on("click", () => {
        let item = add_item("library_add_talent", (placeholder) => {
            $.getJSON("library/add/talent")
                .done((resp) => {
                    let item = adapt_item($(resp.rendered));
                    adapt_form(item.find("form"));
                    placeholder.replaceWith(item);
                    item.find(".card-body").collapse("show");
                })
                .fail(() => {
                    placeholder.remove();
                });
        });
        sheet.prepend(item);
        item.find(".card-body").collapse("show");
    });
});
