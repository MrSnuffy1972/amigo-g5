// AmigoExport.java - Ghidra headless post-script for Ami-Go (13 Oct 2026).
// Args: OUTDIR ENTRY_HEX. Disassembles from the entry point, lets auto-analysis follow the code, then writes
// OUTDIR/functions.txt (entry, name, size, callers) and OUTDIR/listing.txt (every instruction, with labels,
// cross-references and function starts), so a model can read the code instead of hand-decoding bytes.
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.*;
import ghidra.program.model.symbol.*;
import java.io.*;

public class AmigoExport extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] a = getScriptArgs();
        String out = a[0];
        Address entry = toAddr(Long.parseLong(a[1].replace("$", "").replace("0x", ""), 16));
        disassemble(entry);
        if (getFunctionAt(entry) == null) createFunction(entry, "ENTRY");
        analyzeAll(currentProgram);
        Listing l = currentProgram.getListing();
        ReferenceManager rm = currentProgram.getReferenceManager();
        int nf = 0, ni = 0;
        try (PrintWriter pw = new PrintWriter(new FileWriter(out + "/functions.txt"))) {
            for (Function f : currentProgram.getFunctionManager().getFunctions(true)) {
                int callers = 0;
                for (Reference r : rm.getReferencesTo(f.getEntryPoint())) if (r.getReferenceType().isCall()) callers++;
                pw.println(f.getEntryPoint() + "  " + f.getName() + "  size=" + f.getBody().getNumAddresses() + "  callers=" + callers);
                nf++;
            }
        }
        try (PrintWriter pl = new PrintWriter(new FileWriter(out + "/listing.txt"))) {
            for (Instruction i : l.getInstructions(true)) {
                Address ad = i.getAddress();
                Function f = getFunctionAt(ad);
                if (f != null) pl.println("\n; ---- " + f.getName() + " ----");
                Symbol s = getSymbolAt(ad);
                if (s != null && f == null) pl.println(s.getName() + ":");
                StringBuilder refs = new StringBuilder();
                for (Reference r : rm.getReferencesFrom(ad)) {
                    if (r.getReferenceType().isFlow() || r.getReferenceType().isData())
                        refs.append(" ").append(r.getReferenceType().isCall() ? "call " : r.getReferenceType().isData() ? "data " : "jump ").append(r.getToAddress());
                }
                pl.println(ad + "  " + i + (refs.length() > 0 ? "    ;" + refs : ""));
                ni++;
            }
        }
        println("AmigoExport: " + nf + " functions, " + ni + " instructions written to " + out);
    }
}
