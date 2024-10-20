# TODO:
#   - Add checker for journals to be capitalised appropriately
#   - Port to Rust
#   - Check no duplicate IDs
#   - TODO: check no "and others" in the authors
#   - Check that "and" in publisher is expected
#   - Check correct capitalisation of journal
#   - FIX UNUSED COMMAND
#   - check no . at end of title

const INPUT_RE = r"(?<=^%).*\\input\{(?<file>\w+)\}"
const CITE_RE = r"(?<=^%).*\\\w*cite\{(?<id>[\w,\s]+)\}"
const BIB_RE = r"^@(?<type>\w+)\{(?<id>\w+),(?<rest>(?:(?!@).)*)\}"sm
const BIB_KV_RE = r"\s*(?<key>\w+)\s*=\s*\{(?<value>.*)\},?"
const ARTICLE_PAGES_RE = r"^\d+\-\-\d+$"

struct BibEntry
    type::AbstractString
    id::AbstractString
    params::Dict{AbstractString, AbstractString}
end

function find_references(file::AbstractString = "document.tex")
    file_contents = read(file, String)
    captures = [m[:id] for m in eachmatch(CITE_RE, file_contents)]
    citations = Set{String}()
    for m in captures
        for c in eachsplit(m, ",")
            push!(citations, strip(c))
        end
    end
    for m in eachmatch(INPUT_RE, file_contents)
        next_file = m[:file]
        if !endswith(next_file, ".tex")
            next_file *= ".tex"
        end
        union!(citations, find_references(next_file))
    end
    return citations
end

function parse_rest(rest::AbstractString)
    D = Dict{AbstractString, AbstractString}()
    for m in eachmatch(BIB_KV_RE, rest)
        D[m[:key]] = m[:value]
    end
    return D
end

function gather_bib_entries(file::AbstractString = "references.bib")
    file_contents = read(file, String)
    entries = BibEntry[]
    return [BibEntry(m[:type], m[:id], parse_rest(m[:rest]))
            for m in eachmatch(BIB_RE, file_contents)]
end

function find_possible_citations(file::AbstractString = "references.bib")
    return Set{String}(b.id for b in gather_bib_entries(file))
end

function main_unused()
    R, Q = find_references(), find_possible_citations()
    unused = setdiff(Q, R)
    println(join(unused, '\n'))
end

function main_pages()
    entries = gather_bib_entries()
    for b in entries
        if haskey(b.params, "pages")
            pages = b.params["pages"]
            if !occursin(ARTICLE_PAGES_RE, pages)
                println("$(b.id) ($(repr(pages)))")
            end
        end
    end
end

function main_article()
    entries = gather_bib_entries()
    function has_valid_param(b::BibEntry, key::String)
        return haskey(b.params, key) && !isempty(b.params[key])
    end
    function has_valid_params(b::BibEntry, keys::String...)
        return all(has_valid_param(b, k) for k in keys)
    end
    for b in entries
        if b.type == "article"
            if !has_valid_params(b, "volume", "number", "pages", "doi")
                print(b.id)
                print(" (missing: ")
                i = 0
                if !has_valid_params(b, "volume")
                    print("volume")
                    i += 1
                end
                if !has_valid_params(b, "number")
                    i > 0 && print(", ")
                    print("number")
                    i += 1
                end
                if !has_valid_params(b, "pages")
                    i > 0 && print(", ")
                    print("pages")
                    i += 1
                end
                if !has_valid_params(b, "doi")
                    i > 0 && print(", ")
                    print("doi")
                    i += 1
                end
                println(")")
            end
        end
    end
end

function main_collection()
    entries = gather_bib_entries()
    function has_valid_param(b::BibEntry, key::String)
        return haskey(b.params, key) && !isempty(b.params[key])
    end
    function has_valid_params(b::BibEntry, keys::String...)
        return all(has_valid_param(b, k) for k in keys)
    end
    for b in entries
        if b.type == "incollection"
            if !has_valid_params(b, "chapter", "pages", "doi")
                print(b.id)
                print(" (missing: ")
                i = 0
                if !has_valid_params(b, "chapter")
                    print("chapter")
                    i += 1
                end
                if !has_valid_params(b, "pages")
                    i > 0 && print(", ")
                    print("pages")
                    i += 1
                end
                if !has_valid_params(b, "doi")
                    i > 0 && print(", ")
                    print("doi")
                    i += 1
                end
                println(")")
            end
        end
    end
end

function main_book()
    entries = gather_bib_entries()
    function has_valid_param(b::BibEntry, key::String)
        return haskey(b.params, key) && !isempty(b.params[key])
    end
    function has_valid_params(b::BibEntry, keys::String...)
        return all(has_valid_param(b, k) for k in keys)
    end
    for b in entries
        if b.type == "book"
            if !has_valid_params(b, "address", "doi")
                                print(b.id)
                print(" (missing: ")
                i = 0
                if !has_valid_params(b, "address")
                    print("address")
                    i += 1
                end
                if !has_valid_params(b, "doi")
                    i > 0 && print(", ")
                    print("doi")
                    i += 1
                end
                println(")")
            end
        end
    end
end

function main_count()
    counter = Dict{AbstractString, Int}()
    entries = gather_bib_entries()
    for b in entries
        counter[b.type] = get(counter, b.type, 0) + 1
    end
    counts = [(k, v) for (k, v) in counter]
    sort!(counts, by = x -> x[2], rev = true)
    for (k, v) in counts
        println("$k: $v")
    end
end

possible_args = ("count", "unused", "pages", "article", "collection", "book", "help")
possible_args_pretty = join((i == 1 ? "$(a) [default]" : a for (i, a) in enumerate(possible_args)), ", ")

if isempty(ARGS)
    main_count()
elseif length(ARGS) == 1
    arg = only(ARGS)
    if arg ∉ possible_args
        println("Unknown argument \"$arg\".  Supported arguments are: $possible_args_pretty")
        exit(1)
    end
    if arg == "unused"
        main_unused()
    elseif arg == "pages"
        main_pages()
    elseif arg == "article"
        main_article()
    elseif arg == "collection"
        main_collection()
    elseif arg == "book"
        main_book()
    elseif arg == "count"
        main_count()
    elseif arg == "help"
        println("Supported arguments are: $possible_args_pretty")
    else
        println("Unhandled argument \"$arg\"")
        exit(1)
    end
else
    println("We do not support multiple arguments.  Supported arguments are: $possible_args_pretty")
    exit(1)
end

exit(0)
