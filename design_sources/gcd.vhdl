----------------------------------------------------------------------------------
-- Company:
-- Engineer:
--
-- Create Date: 09/12/2024 1 0:05:04 AM
-- Design Name:
-- Module Name: gcd - Behavioral
-- Project Name:
-- Target Devicdes:
-- Tool Versions:
-- Description:
--
-- Dependencies:
--
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
--
----------------------------------------------------------------------------------

library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity gcd is
    Port (
        ack: out STD_LOGIC;
        req: in STD_LOGIC;
        AB: in unsigned(15 downto 0);
        C: out unsigned(15 downto 0);
        -- Clock and Reset
        clk, reset: in STD_LOGIC
    );
end gcd;

ARCHITECTURE fsmd_mealy OF gcd IS
    TYPE StateType IS (
        idle_start, get_A, ack_A, idle_A, get_B, A_greater_than_B, B_greater_than_A, ack_B
    );

    SIGNAL state, next_state : StateType;

    -- Temporary signals for 'wire' simulation
    signal ABorALU_wire, LDA_wire, LDB_wire, N_wire, Z_wire: std_logic;
    signal C_wire, A_wire, B_wire, Y_wire: unsigned(15 downto 0);
    signal FN_wire: std_logic_vector(1 downto 0);

    -- ALU component
    component alu is
    Port (
        A: in unsigned (15 downto 0);
        B: in unsigned (15 downto 0);
        FN: in STD_LOGIC_VECTOR (1 downto 0);
        N: out STD_LOGIC;
        Z: out STD_LOGIC;
        Y: out unsigned (15 downto 0));
    end component;

     -- MUX component
    component mux is
    Port (
        ABorALU: in STD_LOGIC;
        Y: in unsigned (15 downto 0);
        AB: in unsigned (15 downto 0);
        C: out unsigned (15 downto 0));
    end component;

    -- Register A component
    component registerA is
    Port (
        C: in unsigned(15 downto 0);
        LDA: in STD_logic;
        reset: in STD_logic;
        clk: in STD_LOGIC;
        A: out unsigned(15 downto 0)
    );
    end component;

     -- Register B component
    component registerB is
    Port (
        C: in unsigned(15 downto 0);
        LDB: in STD_logic;
        reset: in STD_logic;
        clk: in STD_LOGIC;
        B: out unsigned(15 downto 0)
    );
    end component;


BEGIN

    uut_alu: alu
    Port map(
        A => A_wire,
        B => B_wire,
        FN => FN_wire,
        Y => Y_wire,
        N => N_wire,
        Z => Z_wire
    );

    uut_mux: mux
    Port map(
        ABorALU => ABorALU_wire,
        Y => Y_wire,
        AB => AB,
        C => C_wire
    );

    regA_mod: registerA
    Port map(
        LDA => LDA_wire,
        C => C_wire,
        reset => reset,
        clk => clk,
        A => A_wire
    );

    regB_mod: registerB
    Port map(
        LDB => LDB_wire,
        C => C_wire,
        reset => reset,
        clk => clk,
        B => B_wire
    );

    -- Next state logic
    comb: PROCESS (state,N_wire,Z_wire,req) IS
    BEGIN
        next_state <= state;
        CASE state IS
            when idle_start =>
                if req='1' then
                    next_state <= get_A;
                end if;
            when get_A =>
                next_state <= ack_A;
            when ack_A =>
                next_state <= idle_A;
            when idle_A =>
                if req='1' then
                    next_state <= get_B;
                end if;
            when get_B =>
                next_state <= A_greater_than_B;
            when A_greater_than_B =>
                if N_wire='1' and Z_wire='0' then
                    next_state <= B_greater_than_A;
                elsif Z_wire='1' then
                    next_state <= ack_B;
                end if;
            when B_greater_than_A =>
                if N_wire='1' and Z_wire='0' then
                    next_state <= A_greater_than_B;
                elsif Z_wire='1' then
                    next_state <= ack_B;
                end if;
            when ack_B =>
                next_state <= idle_start;
            -- Robust patch, as per lecture's suggestion if a bit flip happens or a hardware crash, so it could recover!
            when others =>
                next_state <= idle_start;
        END CASE;
    END PROCESS;

    -- Clock and reset logic
    rst_clk: PROCESS (clk,reset) IS
    BEGIN
        if (reset='1') then
            state <= idle_start;
        elsif rising_edge (clk) then
            state <= next_state;
        end if;
    END PROCESS;

    -- Moore output logic
    moore: PROCESS (state, N_wire, Z_wire) -- process(all)
    BEGIN
        ack <= '0';
        ABorALU_wire <= '0';
        LDA_wire <= '0';
        LDB_wire <= '0';
        FN_wire <= "11";

        CASE state IS
            when idle_start =>
            when get_A =>
                ABorALU_wire <= '1';
                LDA_wire <= '1';
            when ack_A =>
                ack <= '1';
            when idle_A =>
            when get_B =>
                ABorALU_wire <= '1';
                LDB_wire <= '1';
            when A_greater_than_B =>
                -- Mealy Machine
                FN_wire <= "00";
                if (N_wire = '0') and (Z_wire = '0') then
                    LDA_wire <= '1';
                end if;
            when B_greater_than_A =>
                FN_wire <= "01";
                if (N_wire = '0') and (Z_wire = '0') then
                    LDB_wire <= '1';
                end if;
            when ack_B =>
                ack <= '1';
        END case;
    END PROCESS;

    C <= C_wire;

END fsmd_mealy;
