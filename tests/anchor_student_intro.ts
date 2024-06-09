import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntro } from "../target/types/anchor_student_intro";
import { expect } from "chai"

describe("anchor_student_intro", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    const program = anchor.workspace.AnchorStudentIntro as Program<AnchorStudentIntro>;

    const intro = {
        name: "Sebastián Jara",
        message: "mensaje de prueba"
    };

    const [introPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(intro.name),
            provider.wallet.publicKey.toBuffer()
        ],
        program.programId
    );

    it("Student intro is added", async () => {
        await program.methods
            .addStudentIntro(
                intro.name,
                intro.message
            )
            .rpc();

        const account = await program.account.introState.fetch(introPDA);
        expect(account.name).to.equal(intro.name);
        expect(account.message).to.equal(intro.message)

    });

    it("Student intro is updated", async () => {
        const newMessage = "otro mensaje de prueba";

        await program.methods
            .updateStudentIntro(intro.name, newMessage)
            .rpc()

        const account = await program.account.introState.fetch(introPDA);
        expect(account.name).to.equal(intro.name);
        expect(account.message).to.equal(newMessage)
    })

    it("Student intro is closed", async () => {
        await program.methods
            .closeStudentIntro(intro.name)
            .rpc()
    })
});
