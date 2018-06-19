# [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::65dac0e0-17b5-4b01-bf3c-7e9bc60a7bde][65dac0e0-17b5-4b01-bf3c-7e9bc60a7bde]]
import ase
import ase.io
import numpy as np
from ase.units import Hartree, eV

# read atoms from calypso generated file
def get_atoms(filename="g.xyz"):
    symbols = []
    positions = []
    for line in open("g.xyz"):
        if not line.strip():
            break
        sym, x, y, z = line.split()
        symbols.append(sym)
        positions.append([x, y, z])

    return ase.Atoms(symbols=symbols, positions=positions)


# optimize the structure using dftb calculator in ase
def set_dftb_calculator(atoms):
    from ase.calculators.dftb import Dftb
    atoms.set_calculator(Dftb(label='c6h6',
                              run_manyDftb_steps=True,
                              Driver_='ConjugateGradient',
                              Driver_MaxForceComponent='1E-3',
                              Driver_MaxSteps=50,
                              atoms=atoms,
                              slako_dir="SKFiles/",
                              Hamiltonian_MaxAngularMomentum_C='"p"',
                              Hamiltonian_MaxAngularMomentum_H='"s"',
    ))

# write output for calypso
def write_gsoutput(atoms, outfile="gsoutput"):
    """write energy and force in mopac aux format.

    Parameters
    ----------
    atoms: ase atoms object
    """
    energy = atoms.get_total_energy()
    # convert to kcal/mol
    print("ase energy = {} eV".format(energy))
    energy = energy*eV/Hartree

    lines = []
    lines.append(" SCF Done:  E(UPBEPBE) = {:20.9f}      A.U. after 0 cycles".format(energy))
    lines.append("                          Input orientation:")
    lines.append(" ---------------------------------------------------------------------")
    lines.append(" Center     Atomic      Atomic             Coordinates (Angstroms)    ")
    lines.append(" Number     Number       Type             X           Y           Z   ")
    lines.append(" ---------------------------------------------------------------------")

    # read optimized geometry
    atoms = ase.io.read("geo_end.gen")

    i = 1
    for a in atoms:
        lines.append(" {:6}{:>12}{:11}{:16.6f}{:12.6f}{:12.6f}".format(i, a.symbol, 0, a.x, a.y, a.z))
        i += 1

    lines.append(" ---------------------------------------------------------------------")
    lines.append("Normal termination of Gaussian 09 at Sat Oct 15 13:07:17 CST 2016.")

    txt = "\n".join(lines)

    with open(outfile, "w") as fp:
        fp.write(txt)

    return txt

def run():
    atoms = get_atoms()
    set_dftb_calculator(atoms)
    write_gsoutput(atoms)

if __name__ == '__main__':
    run()
# 65dac0e0-17b5-4b01-bf3c-7e9bc60a7bde ends here
