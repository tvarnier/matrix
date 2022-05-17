function ToReducedRowEchelonForm(Matrix M) is
    lead := 0
    rowCount := the number of rows in M
    columnCount := the number of columns in M
    for 0 ≤ r < rowCount do
        if columnCount ≤ lead then
            stop function
        end if
        i = r
        while M[i, lead] = 0 do
            i = i + 1
            if rowCount = i then
                i = r
                lead = lead + 1
                if columnCount = lead then
                    stop function
                end if
            end if
        end while
        if i ≠ r then Swap rows i and r
        Divide row r by M[r, lead]
        for 0 ≤ i < rowCount do
            if i ≠ r do
                Subtract M[i, lead] multiplied by row r from row i
            end if
        end for
        lead = lead + 1
    end for
end function
